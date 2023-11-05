package sha1

import (
	"encoding/binary"
	"math/bits"
)

const (
	SIZE  = 20 // bytes; 160 bits
	CHUNK = 64

	init0 = 0x67452301
	init1 = 0xEFCDAB89
	init2 = 0x98BADCFE
	init3 = 0x10325476
	init4 = 0xC3D2E1F0

	K0    = 0x5A827999
	K1    = 0x6ED9EBA1
	K2    = 0x8F1BBCDC
	K3    = 0xCA62C1D6
)

type Digest struct {
	h   [5]uint32
	x   [CHUNK]byte
	nx  uint
	len uint64
}

func New() *Digest {
	return &Digest{
		h:   [5]uint32{0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0},
		nx:  0,
		len: 0,
	}
}

func (self *Digest) Write(p []byte) {
	self.len += uint64(len(p))
	if self.nx > 0 {
		n := uint(copy(self.x[self.nx:], p))
		self.nx += n
		if self.nx == CHUNK {
			self.block(self.x[:])
			self.nx = 0
		}
		p = p[n:]
	}
	if len(p) >= CHUNK {
		n := len(p) & (^(CHUNK - 1))
		self.block(p[:n])
		p = p[n:]
	}
	if len(p) > 0 {
		self.nx = uint(copy(self.x[:], p))
	}
}

func (self *Digest) Sum() [SIZE]byte {
	len := self.len

	var tmp [64 + 8]byte
	tmp[0] = 0x80
	var t uint64
	if len%64 < 56 {
		t = 56 - len%64
	} else {
		t = 64 + 56 - len%64
	}

	len <<= 3
	binary.BigEndian.PutUint64(tmp[t:t+8], len)
	self.Write(tmp[:t+8])

	if self.nx != 0 {panic("self.nx != 0")}

	var digest [SIZE]byte
	binary.BigEndian.PutUint32(digest[0:],  self.h[0])
	binary.BigEndian.PutUint32(digest[4:],  self.h[1])
	binary.BigEndian.PutUint32(digest[8:],  self.h[2])
	binary.BigEndian.PutUint32(digest[12:], self.h[3])
	binary.BigEndian.PutUint32(digest[16:], self.h[4])

	return digest
}

func (self *Digest) block(p []byte) {
	var w [16]uint32

	h0, h1, h2, h3, h4 := self.h[0], self.h[1], self.h[2], self.h[3], self.h[4]
	for len(p) >= CHUNK {
		for i := 0; i < 16; i++ {
			j := i * 4
			w[i] = uint32(p[j]) << 24 | uint32(p[j+1]) << 16 | uint32(p[j+2]) << 8 | uint32(p[j+3])
		}

		a, b, c, d, e := h0, h1, h2, h3, h4

		for i := 0; i < 16; i++ {
			f := b&c | (^b)&d
			t := bits.RotateLeft32(a, 5) + f + e + w[i&0xf] + K0
			a, b, c, d, e = t, a, bits.RotateLeft32(b, 30), c, d
		}
		for i := 16; i < 20; i++ {
			tmp := w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf]
			w[i&0xf] = bits.RotateLeft32(tmp, 1)

			f := b&c | (^b)&d
			t := bits.RotateLeft32(a, 5) + f + e + w[i&0xf] + K0
			a, b, c, d, e = t, a, bits.RotateLeft32(b, 30), c, d
		}
		for i := 20; i < 40; i++ {
			tmp := w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf]
			w[i&0xf] = bits.RotateLeft32(tmp, 1)
			f := b ^ c ^ d
			t := bits.RotateLeft32(a, 5) + f + e + w[i&0xf] + K1
			a, b, c, d, e = t, a, bits.RotateLeft32(b, 30), c, d
		}
		for i := 40; i < 60; i++ {
			tmp := w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf]
			w[i&0xf] = bits.RotateLeft32(tmp, 1)
			f := ((b | c) & d) | (b & c)
			t := bits.RotateLeft32(a, 5) + f + e + w[i&0xf] + K2
			a, b, c, d, e = t, a, bits.RotateLeft32(b, 30), c, d
		}
		for i := 60; i < 80; i++ {
			tmp := w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf]
			w[i&0xf] = bits.RotateLeft32(tmp, 1)
			f := b ^ c ^ d
			t := bits.RotateLeft32(a, 5) + f + e + w[i&0xf] + K3
			a, b, c, d, e = t, a, bits.RotateLeft32(b, 30), c, d
		}

		h0 += a
		h1 += b
		h2 += c
		h3 += d
		h4 += e

		p = p[CHUNK:]
	}

	self.h[0], self.h[1], self.h[2], self.h[3], self.h[4] = h0, h1, h2, h3, h4
}
