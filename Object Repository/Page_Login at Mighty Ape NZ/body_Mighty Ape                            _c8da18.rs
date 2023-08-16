<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Mighty Ape                            _c8da18</name>
   <tag></tag>
   <elementGuidId>66b156c0-d372-45d3-a1aa-9fb6659b0d86</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.store-nz.not-logged-in.non-primate.default-theme.adult-content-no.layout-simple</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>2144de7f-1ff1-43dc-9e0c-6d1c4c351885</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>store-nz not-logged-in non-primate default-theme adult-content-no layout-simple</value>
      <webElementGuid>6f052763-3add-442c-9e32-74117f7d7ea7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
        
        
        
        
            
        
            
                
                                        
                            
            Mighty Ape
    
                    
                
            
        
    

                    
&lt;div class=&quot;notice&quot; data-type=&quot;&quot;>&lt;div>&lt;span class=&quot;icon&quot;>&lt;/span>&lt;p class=&quot;message&quot;>&lt;/p>&lt;span class=&quot;close&quot;>&lt;/span>&lt;/div>&lt;/div>


    
            

        
                    
                
                                    
                                                    
                            
                                
                                    
        
                            Please log in to continue
                    
        
            
                
                    Log in
                    
                        
    
        Email address:
    
    
        

            


    
        Password:
    
    
        
    


                        
                            
                                
                                                                    
                                                                
                            
                            
                                Forgotten your password?
                            
                        
                    
                
                
                    Create an account
                    
                        It's easy, free and only takes a moment.
                    
                    
                        
                            Create an account
                        
                    
                    Creating an account enables you to:
                    
                        Speed through the Checkout
                        Check the status of your orders
                        Create Wish Lists
                        Rate and review products
                        and more...
                    
                
            
        

            
                            
                                            
                            
        
                    
    
        
            
                
                    ×
                    Adult Content Settings
                
                
                
                    Save Settings
                
            
        
    

            
    
        
            
                ×
                Your delivery destination
                
                    Choose a destination to see accurate shipping times, delivery options, pricing and product availability.
                
            
            
        
    


                            
    
        
            
                ×
            
            
            
        
    

            
                    
            
        
        
            
                
                
                    
                                                    All prices are shown in New Zealand Dollars and include GST unless otherwise stated.
                        
                        
                        Copyright © 1996 - 2023 Mighty Ape Limited.


The words &quot;Mighty Ape&quot; and the Ape device are registered trademarks of Mighty Ape Limited in New Zealand and Australia.
All other trademarks are the property of their respective owners.
                    
                
            
        
    

        
            
        
        

                            
                        
            
            
            
            
            
            
            
            
            
            
            
            
            
                $(function () {
                    MA.header.initNav();
                    MA.header.initSearch('/search/autocomplete');
                    MA.header.initBouncingEmailOk();
                    MA.notices.init();
                    MA.utils.dropdowns.init();
                    MA.utils.initResizeWrapper();
                    MA.pageState.init();
                    MA.lastChanceDealsBubble.init();
                    MA.tooltips.init();
                    MA.forms.init();
                    MA.accordion('footer nav.menu-accordion');
                    MA.adultContent.init();
                    MA.locationPicker.init();
                });
            
            
                window.fbAsyncInit = function() {
                    FB.init({
                        appId: '112951975411766',
                        status: true,
                        cookie: true,
                        xfbml: true,
                        version: 'v16.0',
                    });
                };
            
            

                            
                
                    $(function () {
                        MA.primateSubscribeModal.init();
                    });
                

                            
            
            
            
    
        &lt;div class=&quot;footer&quot;>
            Can't find it? &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter your location manually&lt;/button>
        &lt;/div>
    
    
        &lt;div class=&quot;loading&quot;>&lt;/div>
    
    
        &lt;div class=&quot;not-found&quot;>
            &lt;p>
                &lt;span class=&quot;icon&quot;>&lt;/span>
                &lt;b>Sorry, we couldn't find that location.&lt;/b>
            &lt;/p>
            &lt;p>
                &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter Location Manually&lt;/button>
            &lt;/p>
        &lt;/div>
        &lt;div class=&quot;unavailable&quot;>
            &lt;h3>
                &lt;span class=&quot;icon&quot;>&lt;/span>
                Oops! We're having trouble searching the location database.
            &lt;/h3>
            &lt;p>
                Sorry for the inconvenience. Please try again in a moment.
            &lt;/p>
            &lt;p>
                If it's still not working, you can enter your location manually.
            &lt;/p>
            &lt;p>
                &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter Location Manually&lt;/button>
            &lt;/p>
        &lt;/div>
    
    
        &lt;div class=&quot;location-suggestion&quot;>
            {{#each lines}}
                &lt;div class=&quot;line&quot;>{{ this }}&lt;/div>
            {{/each}}
        &lt;/div>
    

        
    
    
        $(function () {
            MA.loginForm.init();
        });
    

                                                        
                    
                
                Kia ora!👋Our support team is online and ready to give you a hand.We usually reply in a few minutesHow can we help you today?Change an orderTrack an order or ETAPayment methods &amp; delivery ratesPick up my order in AucklandInformation about a productDamaged items &amp; ReturnsMinimum order valuesSomething else
                
                    window.MA.chatConfig = {
    &quot;zendeskAccountKey&quot;: &quot;3kaRlvkf9IBKxJtt6wm9VIhapUgBFWyX&quot;,
    &quot;scriptUrl&quot;: &quot;https:\/\/dbari23r2svog.cloudfront.net\/vendor\/mightyape-chat\/main~3503495f.js&quot;,
    &quot;storeUrl&quot;: &quot;https:\/\/www.mightyape.co.nz&quot;,
    &quot;user&quot;: null,
    &quot;baseDeliveryFee&quot;: &quot;5.95&quot;,
    &quot;covid19&quot;: {
        &quot;isHeaderHighVolumeWarningEnabled&quot;: false,
        &quot;isCheckoutDisabled&quot;: false,
        &quot;isCheckoutEssentialsOnly&quot;: false,
        &quot;isPickupsCheckoutDisabled&quot;: false,
        &quot;isPickupsDispatchContactlessPayment&quot;: false,
        &quot;isPickupsDispatchContactless&quot;: false,
        &quot;isPickupsDispatchDisabled&quot;: false
    }
};
                
                
                    
            
        var certona = {
            pagetype: 'login',
            devicetype: 'desktop',
            customerid: '',
            filter: {
                currency: 'NZD',
                countrycode: 'NZ',
                userType: '',
            },
        };
    

        
            MA.recommendations.recommendationsUrl = '/_certona/recommendations';

            var certonaRecommendations = function(data) {
                MA.recommendations.renderCertonaRecommendations(data);
            }
        
        

        
        
            {
    &quot;@context&quot;: &quot;https:\/\/schema.org&quot;,
    &quot;@type&quot;: &quot;Organization&quot;,
    &quot;url&quot;: &quot;https:\/\/www.mightyape.co.nz&quot;,
    &quot;logo&quot;: &quot;https:\/\/dbari23r2svog.cloudfront.net\/frontend\/images\/avatar~a6e963dd.png&quot;,
    &quot;sameAs&quot;: [
        &quot;https:\/\/www.facebook.com\/MightyApe&quot;,
        &quot;https:\/\/www.twitter.com\/MightyApe&quot;,
        &quot;https:\/\/plus.google.com\/+mightyape\/&quot;,
        &quot;https:\/\/www.youtube.com\/user\/mightyape&quot;,
        &quot;https:\/\/instagram.com\/mightyape\/&quot;
    ]
}
        

            

var owl=$(&quot;section.rotisserie \x3e .owl-carousel&quot;);owl.on(&quot;changed.owl.carousel&quot;,function(a){a.namespace&amp;&amp;&quot;position&quot;===a.property.name&amp;&amp;(a=a.relatedTarget.relative(a.property.value)+1,window.dataLayer=window.dataLayer||[],window.dataLayer.push({event:&quot;carouselChange&quot;,owl_position:a}))});google_tag_manager[&quot;rm&quot;][&quot;89370257&quot;](23)(&quot;fbhits&quot;,google_tag_manager[&quot;rm&quot;][&quot;89370257&quot;](25),void 0,&quot;/&quot;,&quot;mightyape.co.nz&quot;);id(&quot;loginForm&quot;)/div[@class=&quot;form-group&quot;]/div[@class=&quot;label-container&quot;]</value>
      <webElementGuid>2e8f3527-9bff-4131-8bd5-5a10e535cad8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js&quot;]/body[@class=&quot;store-nz not-logged-in non-primate default-theme adult-content-no layout-simple&quot;]</value>
      <webElementGuid>4786ba15-deb0-4528-9e28-e9f1b6c1d8b7</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>88106207-6e18-4808-8cbc-00dfe284610e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
        
        
        
        
        
            
        
            
                
                                        
                            
            Mighty Ape
    
                    
                
            
        
    

                    
&lt;div class=&quot;notice&quot; data-type=&quot;&quot;>&lt;div>&lt;span class=&quot;icon&quot;>&lt;/span>&lt;p class=&quot;message&quot;>&lt;/p>&lt;span class=&quot;close&quot;>&lt;/span>&lt;/div>&lt;/div>


    
            

        
                    
                
                                    
                                                    
                            
                                
                                    
        
                            Please log in to continue
                    
        
            
                
                    Log in
                    
                        
    
        Email address:
    
    
        

            


    
        Password:
    
    
        
    


                        
                            
                                
                                                                    
                                                                
                            
                            
                                Forgotten your password?
                            
                        
                    
                
                
                    Create an account
                    
                        It&quot; , &quot;'&quot; , &quot;s easy, free and only takes a moment.
                    
                    
                        
                            Create an account
                        
                    
                    Creating an account enables you to:
                    
                        Speed through the Checkout
                        Check the status of your orders
                        Create Wish Lists
                        Rate and review products
                        and more...
                    
                
            
        

            
                            
                                            
                            
        
                    
    
        
            
                
                    ×
                    Adult Content Settings
                
                
                
                    Save Settings
                
            
        
    

            
    
        
            
                ×
                Your delivery destination
                
                    Choose a destination to see accurate shipping times, delivery options, pricing and product availability.
                
            
            
        
    


                            
    
        
            
                ×
            
            
            
        
    

            
                    
            
        
        
            
                
                
                    
                                                    All prices are shown in New Zealand Dollars and include GST unless otherwise stated.
                        
                        
                        Copyright © 1996 - 2023 Mighty Ape Limited.


The words &quot;Mighty Ape&quot; and the Ape device are registered trademarks of Mighty Ape Limited in New Zealand and Australia.
All other trademarks are the property of their respective owners.
                    
                
            
        
    

        
            
        
        

                            
                        
            
            
            
            
            
            
            
            
            
            
            
            
            
                $(function () {
                    MA.header.initNav();
                    MA.header.initSearch(&quot; , &quot;'&quot; , &quot;/search/autocomplete&quot; , &quot;'&quot; , &quot;);
                    MA.header.initBouncingEmailOk();
                    MA.notices.init();
                    MA.utils.dropdowns.init();
                    MA.utils.initResizeWrapper();
                    MA.pageState.init();
                    MA.lastChanceDealsBubble.init();
                    MA.tooltips.init();
                    MA.forms.init();
                    MA.accordion(&quot; , &quot;'&quot; , &quot;footer nav.menu-accordion&quot; , &quot;'&quot; , &quot;);
                    MA.adultContent.init();
                    MA.locationPicker.init();
                });
            
            
                window.fbAsyncInit = function() {
                    FB.init({
                        appId: &quot; , &quot;'&quot; , &quot;112951975411766&quot; , &quot;'&quot; , &quot;,
                        status: true,
                        cookie: true,
                        xfbml: true,
                        version: &quot; , &quot;'&quot; , &quot;v16.0&quot; , &quot;'&quot; , &quot;,
                    });
                };
            
            

                            
                
                    $(function () {
                        MA.primateSubscribeModal.init();
                    });
                

                            
            
            
            
    
        &lt;div class=&quot;footer&quot;>
            Can&quot; , &quot;'&quot; , &quot;t find it? &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter your location manually&lt;/button>
        &lt;/div>
    
    
        &lt;div class=&quot;loading&quot;>&lt;/div>
    
    
        &lt;div class=&quot;not-found&quot;>
            &lt;p>
                &lt;span class=&quot;icon&quot;>&lt;/span>
                &lt;b>Sorry, we couldn&quot; , &quot;'&quot; , &quot;t find that location.&lt;/b>
            &lt;/p>
            &lt;p>
                &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter Location Manually&lt;/button>
            &lt;/p>
        &lt;/div>
        &lt;div class=&quot;unavailable&quot;>
            &lt;h3>
                &lt;span class=&quot;icon&quot;>&lt;/span>
                Oops! We&quot; , &quot;'&quot; , &quot;re having trouble searching the location database.
            &lt;/h3>
            &lt;p>
                Sorry for the inconvenience. Please try again in a moment.
            &lt;/p>
            &lt;p>
                If it&quot; , &quot;'&quot; , &quot;s still not working, you can enter your location manually.
            &lt;/p>
            &lt;p>
                &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter Location Manually&lt;/button>
            &lt;/p>
        &lt;/div>
    
    
        &lt;div class=&quot;location-suggestion&quot;>
            {{#each lines}}
                &lt;div class=&quot;line&quot;>{{ this }}&lt;/div>
            {{/each}}
        &lt;/div>
    

        
    
    
        $(function () {
            MA.loginForm.init();
        });
    

                                                        
                    
                
                Kia ora!👋Our support team is online and ready to give you a hand.We usually reply in a few minutesHow can we help you today?Change an orderTrack an order or ETAPayment methods &amp; delivery ratesPick up my order in AucklandInformation about a productDamaged items &amp; ReturnsMinimum order valuesSomething else
                
                    window.MA.chatConfig = {
    &quot;zendeskAccountKey&quot;: &quot;3kaRlvkf9IBKxJtt6wm9VIhapUgBFWyX&quot;,
    &quot;scriptUrl&quot;: &quot;https:\/\/dbari23r2svog.cloudfront.net\/vendor\/mightyape-chat\/main~3503495f.js&quot;,
    &quot;storeUrl&quot;: &quot;https:\/\/www.mightyape.co.nz&quot;,
    &quot;user&quot;: null,
    &quot;baseDeliveryFee&quot;: &quot;5.95&quot;,
    &quot;covid19&quot;: {
        &quot;isHeaderHighVolumeWarningEnabled&quot;: false,
        &quot;isCheckoutDisabled&quot;: false,
        &quot;isCheckoutEssentialsOnly&quot;: false,
        &quot;isPickupsCheckoutDisabled&quot;: false,
        &quot;isPickupsDispatchContactlessPayment&quot;: false,
        &quot;isPickupsDispatchContactless&quot;: false,
        &quot;isPickupsDispatchDisabled&quot;: false
    }
};
                
                
                    
            
        var certona = {
            pagetype: &quot; , &quot;'&quot; , &quot;login&quot; , &quot;'&quot; , &quot;,
            devicetype: &quot; , &quot;'&quot; , &quot;desktop&quot; , &quot;'&quot; , &quot;,
            customerid: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            filter: {
                currency: &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot;,
                countrycode: &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;,
                userType: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            },
        };
    

        
            MA.recommendations.recommendationsUrl = &quot; , &quot;'&quot; , &quot;/_certona/recommendations&quot; , &quot;'&quot; , &quot;;

            var certonaRecommendations = function(data) {
                MA.recommendations.renderCertonaRecommendations(data);
            }
        
        

        
        
            {
    &quot;@context&quot;: &quot;https:\/\/schema.org&quot;,
    &quot;@type&quot;: &quot;Organization&quot;,
    &quot;url&quot;: &quot;https:\/\/www.mightyape.co.nz&quot;,
    &quot;logo&quot;: &quot;https:\/\/dbari23r2svog.cloudfront.net\/frontend\/images\/avatar~a6e963dd.png&quot;,
    &quot;sameAs&quot;: [
        &quot;https:\/\/www.facebook.com\/MightyApe&quot;,
        &quot;https:\/\/www.twitter.com\/MightyApe&quot;,
        &quot;https:\/\/plus.google.com\/+mightyape\/&quot;,
        &quot;https:\/\/www.youtube.com\/user\/mightyape&quot;,
        &quot;https:\/\/instagram.com\/mightyape\/&quot;
    ]
}
        

            

var owl=$(&quot;section.rotisserie \x3e .owl-carousel&quot;);owl.on(&quot;changed.owl.carousel&quot;,function(a){a.namespace&amp;&amp;&quot;position&quot;===a.property.name&amp;&amp;(a=a.relatedTarget.relative(a.property.value)+1,window.dataLayer=window.dataLayer||[],window.dataLayer.push({event:&quot;carouselChange&quot;,owl_position:a}))});google_tag_manager[&quot;rm&quot;][&quot;89370257&quot;](23)(&quot;fbhits&quot;,google_tag_manager[&quot;rm&quot;][&quot;89370257&quot;](25),void 0,&quot;/&quot;,&quot;mightyape.co.nz&quot;);id(&quot;loginForm&quot;)/div[@class=&quot;form-group&quot;]/div[@class=&quot;label-container&quot;]&quot;) or . = concat(&quot;
        
        
        
        
        
            
        
            
                
                                        
                            
            Mighty Ape
    
                    
                
            
        
    

                    
&lt;div class=&quot;notice&quot; data-type=&quot;&quot;>&lt;div>&lt;span class=&quot;icon&quot;>&lt;/span>&lt;p class=&quot;message&quot;>&lt;/p>&lt;span class=&quot;close&quot;>&lt;/span>&lt;/div>&lt;/div>


    
            

        
                    
                
                                    
                                                    
                            
                                
                                    
        
                            Please log in to continue
                    
        
            
                
                    Log in
                    
                        
    
        Email address:
    
    
        

            


    
        Password:
    
    
        
    


                        
                            
                                
                                                                    
                                                                
                            
                            
                                Forgotten your password?
                            
                        
                    
                
                
                    Create an account
                    
                        It&quot; , &quot;'&quot; , &quot;s easy, free and only takes a moment.
                    
                    
                        
                            Create an account
                        
                    
                    Creating an account enables you to:
                    
                        Speed through the Checkout
                        Check the status of your orders
                        Create Wish Lists
                        Rate and review products
                        and more...
                    
                
            
        

            
                            
                                            
                            
        
                    
    
        
            
                
                    ×
                    Adult Content Settings
                
                
                
                    Save Settings
                
            
        
    

            
    
        
            
                ×
                Your delivery destination
                
                    Choose a destination to see accurate shipping times, delivery options, pricing and product availability.
                
            
            
        
    


                            
    
        
            
                ×
            
            
            
        
    

            
                    
            
        
        
            
                
                
                    
                                                    All prices are shown in New Zealand Dollars and include GST unless otherwise stated.
                        
                        
                        Copyright © 1996 - 2023 Mighty Ape Limited.


The words &quot;Mighty Ape&quot; and the Ape device are registered trademarks of Mighty Ape Limited in New Zealand and Australia.
All other trademarks are the property of their respective owners.
                    
                
            
        
    

        
            
        
        

                            
                        
            
            
            
            
            
            
            
            
            
            
            
            
            
                $(function () {
                    MA.header.initNav();
                    MA.header.initSearch(&quot; , &quot;'&quot; , &quot;/search/autocomplete&quot; , &quot;'&quot; , &quot;);
                    MA.header.initBouncingEmailOk();
                    MA.notices.init();
                    MA.utils.dropdowns.init();
                    MA.utils.initResizeWrapper();
                    MA.pageState.init();
                    MA.lastChanceDealsBubble.init();
                    MA.tooltips.init();
                    MA.forms.init();
                    MA.accordion(&quot; , &quot;'&quot; , &quot;footer nav.menu-accordion&quot; , &quot;'&quot; , &quot;);
                    MA.adultContent.init();
                    MA.locationPicker.init();
                });
            
            
                window.fbAsyncInit = function() {
                    FB.init({
                        appId: &quot; , &quot;'&quot; , &quot;112951975411766&quot; , &quot;'&quot; , &quot;,
                        status: true,
                        cookie: true,
                        xfbml: true,
                        version: &quot; , &quot;'&quot; , &quot;v16.0&quot; , &quot;'&quot; , &quot;,
                    });
                };
            
            

                            
                
                    $(function () {
                        MA.primateSubscribeModal.init();
                    });
                

                            
            
            
            
    
        &lt;div class=&quot;footer&quot;>
            Can&quot; , &quot;'&quot; , &quot;t find it? &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter your location manually&lt;/button>
        &lt;/div>
    
    
        &lt;div class=&quot;loading&quot;>&lt;/div>
    
    
        &lt;div class=&quot;not-found&quot;>
            &lt;p>
                &lt;span class=&quot;icon&quot;>&lt;/span>
                &lt;b>Sorry, we couldn&quot; , &quot;'&quot; , &quot;t find that location.&lt;/b>
            &lt;/p>
            &lt;p>
                &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter Location Manually&lt;/button>
            &lt;/p>
        &lt;/div>
        &lt;div class=&quot;unavailable&quot;>
            &lt;h3>
                &lt;span class=&quot;icon&quot;>&lt;/span>
                Oops! We&quot; , &quot;'&quot; , &quot;re having trouble searching the location database.
            &lt;/h3>
            &lt;p>
                Sorry for the inconvenience. Please try again in a moment.
            &lt;/p>
            &lt;p>
                If it&quot; , &quot;'&quot; , &quot;s still not working, you can enter your location manually.
            &lt;/p>
            &lt;p>
                &lt;button type=&quot;button&quot; class=&quot;enter-manually&quot;>Enter Location Manually&lt;/button>
            &lt;/p>
        &lt;/div>
    
    
        &lt;div class=&quot;location-suggestion&quot;>
            {{#each lines}}
                &lt;div class=&quot;line&quot;>{{ this }}&lt;/div>
            {{/each}}
        &lt;/div>
    

        
    
    
        $(function () {
            MA.loginForm.init();
        });
    

                                                        
                    
                
                Kia ora!👋Our support team is online and ready to give you a hand.We usually reply in a few minutesHow can we help you today?Change an orderTrack an order or ETAPayment methods &amp; delivery ratesPick up my order in AucklandInformation about a productDamaged items &amp; ReturnsMinimum order valuesSomething else
                
                    window.MA.chatConfig = {
    &quot;zendeskAccountKey&quot;: &quot;3kaRlvkf9IBKxJtt6wm9VIhapUgBFWyX&quot;,
    &quot;scriptUrl&quot;: &quot;https:\/\/dbari23r2svog.cloudfront.net\/vendor\/mightyape-chat\/main~3503495f.js&quot;,
    &quot;storeUrl&quot;: &quot;https:\/\/www.mightyape.co.nz&quot;,
    &quot;user&quot;: null,
    &quot;baseDeliveryFee&quot;: &quot;5.95&quot;,
    &quot;covid19&quot;: {
        &quot;isHeaderHighVolumeWarningEnabled&quot;: false,
        &quot;isCheckoutDisabled&quot;: false,
        &quot;isCheckoutEssentialsOnly&quot;: false,
        &quot;isPickupsCheckoutDisabled&quot;: false,
        &quot;isPickupsDispatchContactlessPayment&quot;: false,
        &quot;isPickupsDispatchContactless&quot;: false,
        &quot;isPickupsDispatchDisabled&quot;: false
    }
};
                
                
                    
            
        var certona = {
            pagetype: &quot; , &quot;'&quot; , &quot;login&quot; , &quot;'&quot; , &quot;,
            devicetype: &quot; , &quot;'&quot; , &quot;desktop&quot; , &quot;'&quot; , &quot;,
            customerid: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            filter: {
                currency: &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot;,
                countrycode: &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;,
                userType: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            },
        };
    

        
            MA.recommendations.recommendationsUrl = &quot; , &quot;'&quot; , &quot;/_certona/recommendations&quot; , &quot;'&quot; , &quot;;

            var certonaRecommendations = function(data) {
                MA.recommendations.renderCertonaRecommendations(data);
            }
        
        

        
        
            {
    &quot;@context&quot;: &quot;https:\/\/schema.org&quot;,
    &quot;@type&quot;: &quot;Organization&quot;,
    &quot;url&quot;: &quot;https:\/\/www.mightyape.co.nz&quot;,
    &quot;logo&quot;: &quot;https:\/\/dbari23r2svog.cloudfront.net\/frontend\/images\/avatar~a6e963dd.png&quot;,
    &quot;sameAs&quot;: [
        &quot;https:\/\/www.facebook.com\/MightyApe&quot;,
        &quot;https:\/\/www.twitter.com\/MightyApe&quot;,
        &quot;https:\/\/plus.google.com\/+mightyape\/&quot;,
        &quot;https:\/\/www.youtube.com\/user\/mightyape&quot;,
        &quot;https:\/\/instagram.com\/mightyape\/&quot;
    ]
}
        

            

var owl=$(&quot;section.rotisserie \x3e .owl-carousel&quot;);owl.on(&quot;changed.owl.carousel&quot;,function(a){a.namespace&amp;&amp;&quot;position&quot;===a.property.name&amp;&amp;(a=a.relatedTarget.relative(a.property.value)+1,window.dataLayer=window.dataLayer||[],window.dataLayer.push({event:&quot;carouselChange&quot;,owl_position:a}))});google_tag_manager[&quot;rm&quot;][&quot;89370257&quot;](23)(&quot;fbhits&quot;,google_tag_manager[&quot;rm&quot;][&quot;89370257&quot;](25),void 0,&quot;/&quot;,&quot;mightyape.co.nz&quot;);id(&quot;loginForm&quot;)/div[@class=&quot;form-group&quot;]/div[@class=&quot;label-container&quot;]&quot;))]</value>
      <webElementGuid>eb35e6ba-45ce-405e-aec0-65bed8217242</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
