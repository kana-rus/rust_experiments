export const SAMPLE_MESSAGE = {
    id: 777,
    platformMessageType: "gmail",
    title: "nosh sample",
    type: "html",
    sentTime: new Date(),
    sender: { name: "kanarus <kanarus786@gmail.com>" },
    repliers: [],
    integrationName: "kanarus786@gmail.com",
    text: `<!DOCTYPE html>
<html lang="ja-JP">
    <head>
        <meta charset="utf-8">
        <title></title>
        <meta name="viewport" content="width=device-width,initial-scale=1">
        <meta http-equiv="content-style-type" content="text/css" />
        <style type="text/css">
            * {
margin: 0 !important;
padding: 0 !important;
}

a {
color: #63A351 !important;
}

body {
color: #535353 !important;
font-size: 12px !important;
line-height: 2 !important;
font-family: "Hiragino Kaku Gothic Pro", Meiryo !important;
}

@media (prefers-dark-interface) {
body {
background-color: #FFFFFE !important;
-apple-color-filter: none;
}
}

table {
margin: 0 auto !important;
}

img {
border: none !important;
vertical-align: top !important;
margin: 0 !important;
padding: 0 !important;
}

hr {
display: none !important;
}

ul {
list-style-type: none !important;
}

#wraper {
width: 600px !important;
margin: 0px auto !important;
}

#bar {
background-color: #63A351 !important;
height: 4px !important;
width: 600px !important;
margin: 0 auto !important;
}

#header {
width: 600px !important;
padding: 0px 0 0 0 !important;
margin: 0px auto !important;
text-align: center !important;
background-color: #FFFFFE !important;
}

#header p.logo {
padding: 20px 0px !important;
float: left !important;
}

#header p.view {
padding: 20px 0px !important;
float: right !important;
}

#info {
width: 600px !important;
margin: 0px auto !important;
padding: 0 0 50px 0 !important;
border-bottom: solid 1px #BBBBBB !important;
}

#info.ambassador_info {
    border-bottom: none !important;
}

#info h1 {
font-size: 16px !important;
font-weight: normal !important;
padding: 20px 0 !important;
}

#greeting {
padding: 0 0 40px 0 !important;
}

#list {
padding: 30px 0 30px 0 !important;
text-align: center !important;
clear: both !important;
}

#end {
padding: 30px 0 0 0 !important;
}

#skip ul {
margin: 40px auto !important;
overflow: hidden !important;
text-align: center !important;
}

#skip li {
padding: 0 10px !important;
display: inline !important;
}

#release {
padding: 20px 0 !important;
}

#next-time2 h1 {
font-size: 18px !important;
color: #63A351 !important;
text-align: center !important;
}

#next-time2 h1 span {
color: #63A351 !important;
line-height: 1.2 !important;
font-size: 16px !important;
display: block !important;
}

#next-time2 h1 span.danger {
font-weight: bold !important;
color: #FF0000 !important;
padding: 10px 0px 0px 0px !important;
}

#next-time2 table {
line-height: 1.5 !important;
}

#next-time2 td {
width: 120px !important;
vertical-align: top !important;
}

#next-time2 td span {
padding: 10px 0px !important;
display: block !important;
text-align: center !important;
}

#next-time2 table+p {
text-align: center !important;
}

#next-time2 div.recommend {
height: 830px !important;
margin: 40px 0px !important;
padding: 40px 50px !important;
background: #f9f6d6!important;
}

#next-time2 div.recommend div {
background-color: #FFFFFE !important;
width: 420px !important;
height: 780px !important;
padding: 30px 40px !important;
}

#next-time2 h2 {
color: #52A530 !important;
font-size: 20px !important;
font-weight: normal !important;
}

#next-time2 div.recommend table {
margin: 30px 0 0 0 !important;
width: 420px !important;
}

#next-time2 div.recommend span.item-name {
padding: 10px 0px !important;
height: 20px !important;
color: #63A351 !important;
display: block !important;
}

#next-time2 div.recommend span+span {
padding: 10px 10px 20px 10px !important;
height: auto !important;
color: #535353 !important;
display: block !important;
}

div.come-back-a {
font-size: 20px !important;
padding: 20px 0 0 0 !important;
font-weight: bold !important;
color: #FF0000 !important;
text-align: center !important;
}

div.expired-info {
    font-size: 20px !important;
    padding: 20px 0 0 0 !important;
    font-weight: bold !important;
    text-align: center !important;
}

#next-time2 div.notice-invalidation {
    margin: 20px 0px !important;
    padding: 10px 20px !important;
}

#next-time2 div.notice-invalidation h2{
    text-align: center;
    padding-bottom: 20px !important;
}


div.come-back-b {
font-size: 20px !important;
padding: 20px 0 0 0 !important;
font-weight: bold !important;
text-align: center !important;
}

div.come-back-c {
font-size: 20px !important;
padding: 0px 0 0 0 !important;
font-weight: bold !important;
color: #FFFFFE !important;
background-color: #E63514 !important;
text-align: center !important;
}

.DiscontinuedArea .DiscontinuedArea__title{
display: block;
width: 100%;
text-align: center;
font-size: 16px;
font-weight: bold;
padding: 15px 0 !important;
margin-top: 15px !important;
}

.menu-detail {
background: -o-linear-gradient(bottom, white 92%, #edeae6) !important;
background: -webkit-gradient(linear, left bottom, left top, color-stop(92%,=
 white), to(#edeae6)) !important;
background: linear-gradient(0deg, white 92%, #edeae6) !important;
border-top: 1px solid #c3deae !important;
margin-bottom: 20px !important;
overflow:hidden;
}
.menu-detail__label {
float: left !important;
display: block !important;
width: 50px !important;
height: 50px !important;
}

.menu-detail__top {
padding-left: 60px !important;
margin-bottom: 20px !important;
}

.menu-detail__title {
font-size: 18px !important;
font-weight: bold !important;
color: #39771f !important;
padding-top: 20px !important;
}



.menu-detail__text {
font-size: 14px !important;
line-height: 1.6 !important;
color: #52a530 !important;
}

.menu-detail__img {
width: 100% !important;
}

.menu-detail__img img {
-webkit-box-sizing: border-box !important;
box-sizing: border-box !important;
width: 100% !important;
height: auto !important;
}

.menu-detail__nutrients {
width: 96% !important;
margin: 10px auto 0 !important;
}

.menu-detail__table {
width: 100%;
margin-bottom: 20px !important;
border-collapse: collapse !important;
border-top: 2px solid #52a530 !important;
border-bottom: 2px solid #52a530 !important;
}

.menu-detail__table tr {
border-bottom: 1px solid #ddd !important;
}

.menu-detail__table td.odd {
padding: 5px 10px 5px 0 !important;
border-right: 1px solid #ddd !important;
}

.menu-detail__table td.even {
padding: 5px 0 5px 10px !important;
}

.menu-detail__table .name {
display: inline-block !important;
padding-top: 5px !important;
}

.menu-detail__table .number {
float: right !important;
font-size: 16px !important;
color: #52a530 !important;
}

.menu-detail__table .number.orange {
color: #dc6f05 !important;
}

.menu-detail__menu-link {
text-align: center !important;
}
.menu-detail__menu-link a {
display: inline-block !important;
min-width: 200px !important;
padding: 5px !important;
color: #fff !important;
text-decoration: none !important;
background: #52a530 !important;
border-radius: 30px !important;
}

.menu-detail.discontinued{
width: 29% !important;
margin-top: 10px !important;
padding: 0 2%!important;
overflow:hidden!important;
background: linear-gradient(0deg, white 92%, white) !important;
border-top: 0px solid #c3deae !important;
float:left !important;
}
.menu-detail.discontinued .menu-detail__title{
padding-top: 5px !important;
color:#333333 !important;
font-size:12px !important;
}
.menu-detail.discontinued .menu-detail__img{
width:auto !important;
border: 1px solid #ddd !important;
}
.menu-detail.discontinued .menu-detail__img img{
width: 100% !important;
height: auto !important;
border: 2px solid #fff !important;
}
.menu-detail.discontinued  .menu-detail__top{
background-image:none !important;
padding-left: 0px !important;
margin-bottom: 5px !important;
}

.planchenge-btn {
padding-bottom: 30px !important;
}
#newyear h1 {
font-size: 18px !important;
padding: 20px 0 !important;
color: #52A530 !important;
}

#newyear div {
border-top: solid 3px #D6D6D6 !important;
padding: 0 0 40px 0 !important;
}

#newyear div+div {
border-top: solid 3px #D6D6D6 !important;
border-bottom: solid 3px #D6D6D6 !important;
padding: 0 0 40px 40px !important;
margin: 0 0 40px 0 !important;
background-color: #F7F7F7 !important;
text-align: center !important;
}

.slide__title {
font-size: 16px !important;
}

.slide__date {
color: #52A530 !important;
margin-bottom: 20px !important;
}

#slide h1 {
font-size: 18px !important;
padding: 20px 0 !important;
color: #52A530 !important;
}

#slide div {
border-top: solid 3px #D6D6D6 !important;
border-bottom: solid 3px #D6D6D6 !important;
padding: 0 0 40px 0 !important;
margin: 40px auto !important;
background-color: #F7F7F7 !important;
text-align: center !important;
}

.atobarai-block {
padding: 30px 0 !important;
border: 5px solid #eee !important;
width: 450px !important;
margin: 0 auto !important;
text-align: center !important;
}

.friend-promotion {
    padding-top: 20px !important;
}

.google-review {
    padding-top: 20px !important;
}

.mail-form {
padding: 10px !important;
margin-top: 30px !important;
text-align: center !important;
border: solid 4px #EEEEEE !important;
}

.mail-form-link {
margin-top: 20px !important;
}

.main-noimage {
clear: both !important;
border-top: 1px solid #BBBBBB !important;
}

.shipping-number {
display: inline-block !important;
max-width: 300px !important;
text-align: left !important;
padding: 30px 0 !important;
margin: 0 auto !important;
font-weight: bold !important;
}

#shipping {
padding: 0 0 60px 0 !important;
}

#shipping h1 {
font-size: 18px !important;
padding: 20px 0 !important;
color: #52A530 !important;
}

#substitute p{
padding:0 0 40px 0 !important;
}
#substitute p.quotion{
margin:-20px 0 0 0 !important;
color:#9A0028 !important;
}

#cahnge h1{
padding:20px 0 0 0!important;
}
#cahnge span{
font-size:14px!important;
display:block!important;
padding:10px 0 15px 0!important;
}

#noshclub{
text-align:center!important;
}
#noshclub h1{
border-top:solid 1px #D6D6D6!important;
border-bottom:solid 1px #D6D6D6!important;
font-weight:bold!important;
font-size:20px!important;
padding:5px 0!important;
margin:30px 0 0 0!important;
}
#noshclub h1 span{
font-size:14px!important;
padding:0 0 0 20px!important;
}
#noshclub ul{
overflow:hidden!important;
margin:20px auto!important;
display: flex!important;
justify-content: center!important;
}
#noshclub li {
width:100%!important;
text-align:center!important;
border-right:solid 1px #D6D6D6!important;
}
#noshclub li:last-child{
border-right:none!important;
}
#noshclub li span{
font-size:24px!important;
color:#FF4C65!important;
display:block!important;
}
#noshclub table, #noshclub td, #noshclub th {
white-space: nowrap!important;
width:100%!important;
border-collapse: collapse!important;
border:solid 1px #D6D6D6!important;
font-size:14px!important;
}
#noshclub td, #noshclub th{
width:20%!important;
}
#noshclub td:last-child{
color:#FF4C65!important;
font-size:16px!important;
}
#noshclub th{
background-color:#1F3D08!important;
color:#FFFFFF!important;
font-size:14px!important;
padding:10px!important;
}
#noshclub th.you{
color:#FF4C65!important;
}
#noshclub td{
padding:10px!important;
}
#noshclub tr.you{
background-color:#EEF7E5!important;
}

#footer {
width: 600px !important;
padding: 20px 0 !important;
margin: 0px auto !important;
font-size: 10px !important;
}

#footer p.copyright {
float: left !important;
}

#footer ul.sns {
width: 600px !important;
clear: both !important;
overflow: hidden !important;
padding: 20px 0 0 0 !important;
margin: 20px auto !important;
text-align: center !important;
}

#footer ul.sns li {
display: inline !important;
padding: 0 5px !important;
}

#footer ul.sub {
width: 600px !important;
clear: both !important;
overflow: hidden !important;
margin: 0px auto !important;
text-align: center !important;
}

#footer ul.sub li {
display: inline !important;
padding: 0 10px !important;
}

#footer .ambassador_icon {
    width: 100px;
    margin: 0 auto !important;
    margin-bottom: 20px !important;
}

#footer .ambassador_link {
    width: fit-content;
    margin: 0 auto !important;
}

#footer .ambassador_link .ambassador_link_item {
    color: #79796a !important;
    font-size: .9rem !imoprtnat;
    padding: 0 8px !important;
    text-decoration: none;
    text-align: center !important;
}

#footer .ambassador_copyright {
    margin-top: 32px !important;
    padding: 32px 0 !important;
    border-top: 1px solid #ddd !important;
}

#footer .ambassador_copyright p {
    font-size: 10px !important;
    text-align: center !important;
}

.nosh-club-info p {
    font-size: 24px !important;
    font-weight: 600 !important;
    text-align: center !important;
    padding: 35px 0 !important;
}

.nosh-club-info span {
    color: #d11917;
}

.plan-stop {
    font-size: 20px;
    text-align: center;
}

.skip-feature-desc {
    text-align: center !important;
    padding: 30px 0 30px 0 !important;
    border-top: 1px solid #dddddd !important;
    border-bottom: 1px solid #dddddd !important;
}

.skip-feature-desc-title {
    font-size: 14px !important;
    font-weight: bold !important;
}

.skip-feature-desc-text {
    padding-top: 15px !important;
}
        </style>
    </head>
    <body>
        <div id="bar"></div>

        <div id="wraper">
    <div id="header">
    <p class="logo">
            <img src="https://img.nosh.jp/images/chefly/mail/img_logo.png"/>
    </p>
    <p class="view">
    <a href="https://nosh.jp" target="_blank">
                    <img src="https://img.nosh.jp/images/chefly/mail/bt_view.png"/>
            </a>
</p>
</div>
    <div id="main">
        <p>
                            <img src="https://img.nosh.jp/images/chefly/mail/img_main_shipping.jpg"/>
                    </p>
    </div>
    <div id="info">
        <div id="greeting">
            <br>

            <p>
                いつも「nosh-ナッシュ」をご利用いただきまして誠にありがとうございます。<br>
                <br>
                配送中の商品の伝票番号が確定いたしましたので、以下にお知らせいたします。<br>
                <br>
                伝票番号で追跡可能となるまで、1日程度お時間がかかることがございます。<br>
                「お問い合わせいただいた伝票番号は、今現在コンピュータに登録されておりません」と表示される場合は、お日にち・お時間を改めてご検索ください。<br>
            </p>


            <p class="shipping-number">
                [お荷物伝票番号] 483484399683<br>
                [配送会社] ヤマト運輸<br>
                [お届け先名] 佐々木哉人<br>
                [お届け先]<br>
                6068102<br>
                京都府 京都市左京区高野清水町56 Crescere洛北106号室
            </p>
            <p>
                ※「nosh-ナッシュ」の商品はクール冷凍便でのお届けとなるため、直接のお受け取りをお願いいたします。<br>
                <br>
                ※配送遅延・状況確認・荷物破損など、配送に関するトラブルが発生した場合は、<br>
                担当の配送業者へ、直接お問い合わせをお願いいたします。<br>
                <br>
                ※長期不在および受取拒否により、お客様都合で商品が返送された場合は「返品」扱いとなり、<br>
                利用規約第11条第6項に基づき、原則として商品代金・送料往復分およびその他実費の請求が発生いたします。<br>
                （この場合、割引クーポンは適用されませんのでご了承ください。）<br>
                また、返送された商品は「食品」という特性上、再利用ができないため廃棄となり再送は行っておりません。<br>
                そのため、必ず、クール便保管期限内に配送業者へ再配達依頼を行い、お受け取りをお願いいたします。<br>
                <br>
                商品は、ヤマト運輸にて発送しております。<br>
                以下より配送状況をご確認頂けます。<br>
                <br>

                <a href="http://jizen.kuronekoyamato.co.jp/jizen/servlet/crjz.b.NQ0010?id=3D483484399683">
                        <img src="https://img.nosh.jp/images/chefly/mail/bt_shipment.png"/>
                </a>

                <br>
                それでは、商品の到着までもうしばらくお待ちくださいませ。<br>
                <br>
            </p>
        </div>
        <div id="end">
            <p>どうぞ今後とも 「nosh-ナッシュ」 をよろしくお願いいたします。</p>
        </div>

        <div class="friend-promotion">
            <a href="https://nosh.jp/friend-202103/promotion?utm_source=3Dmail&utm_medium=3Dbanner" target="_blank">
                    <img src="https://img.nosh.jp/images/chefly/mail/friend_promotion_01--20230425.png" alt="" style="display: block; width: 100%;">
            </a>
        </div>

        <div class="google-review">
            <a href="https://g.page/r/CU5VvfZaq-OmEAg/review" target="_blank">
                    <img src="https://img.nosh.jp/images/chefly/mail/google_review_01--20220228.png" alt="" style="display: block; width: 100%;">
            </a>
        </div>

<div class="mail-form">
本メールの送信メールアドレスは送信専用となっております。<br>
ご返信いただいても回答いたしかねます。<br>
ご意見・お問い合わせ、その他なにか気になる点などございましたら、<br>
下記リンクのフォームよりお気兼ねなくご連絡ください。

    <p class="mail-form-link">
        <a href="https://nosh.jp/contact">お問い合わせ</a>
    </p>
</div>
    </div>
</div>

        <div id="footer">
    <p class="copyright">
    〒530-0005<br>
    大阪府大阪市北区中之島3-3-3 中之島三井ビルディング16F<br>
    TEL：<a href="tel:050-3101-6850">050-3101-6850</a> 受付時間：10:00 ～ 19:00 (年末年始を除く)<br>
        WEB=EF=BC=9A<a href="https://nosh.jp">https://nosh.jp</a><br>
        Copyright © 2023 nosh. All Rights Reserved.
    </p>

    <ul class="sns">
        <li>
            <a href="https://www.youtube.com/channel/UC2bxbF_xG7JDmVVuja4U-ow" target="_blank">
                                    <img src="https://img.nosh.jp/images/chefly/mail/i_youtube.png" width="27" height="20"/>
                            </a>
        </li>
        <li>
            <a href="https://www.facebook.com/plus.nosh.jp/" target="_blank">
                                    <img src="https://img.nosh.jp/images/chefly/mail/i_facebook.png" width="20" height="20"/>
                            </a>
        </li>
        <li>
            <a href="https://lin.ee/stBSbaP" target="_blank">
                                    <img src="https://img.nosh.jp/images/chefly/mail/i_line.png" width="20" height="20"/>
                            </a>
        </li>
        <li>
            <a href="https://twitter.com/nosh_fresh?lang=3Dja" target="_blank">
                                    <img src="https://img.nosh.jp/images/chefly/mail/i_twitter.png" width="20" height="20"/>
                            </a>
        </li>
        <li>
            <a href="https://www.instagram.com/nosh_fresh/" target="_blank">
                                    <img src="https://img.nosh.jp/images/chefly/mail/i_insta.png" width="20" height="20"/>
                            </a>
        </li>
        <li>
            <a href="https://www.pinterest.jp/nosh_0040/" target="_blank">
                                    <img src="https://img.nosh.jp/images/chefly/mail/i_pinta.png" width="20" height="20"/>
                            </a>
        </li>
    </ul>
    <ul class="sub">
        <li>
            <a href="https://nosh.jp/faq">よくある質問</a>
        </li>
        <li>
            <a href="https://nosh.jp/company">会社概要</a>
        </li>
    </ul>
</div>
    </body>
</html>`,
};
