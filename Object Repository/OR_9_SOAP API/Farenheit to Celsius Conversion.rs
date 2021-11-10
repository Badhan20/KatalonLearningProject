<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Farenheit to Celsius Conversion</name>
   <tag></tag>
   <elementGuidId>29740461-5268-40e2-bddb-5bd29312b676</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/soap+xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;utf-8&quot;?>
&lt;soap12:Envelope xmlns:xsi=&quot;http://www.w3.org/2001/XMLSchema-instance&quot; xmlns:xsd=&quot;http://www.w3.org/2001/XMLSchema&quot; xmlns:soap12=&quot;http://www.w3.org/2003/05/soap-envelope&quot;>
  &lt;soap12:Body>
    &lt;FahrenheitToCelsius xmlns=&quot;https://www.w3schools.com/xml/&quot;>
      &lt;Fahrenheit>${FarenheitInput}&lt;/Fahrenheit>
    &lt;/FahrenheitToCelsius>
  &lt;/soap12:Body>
&lt;/soap12:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.w3schools.com/xml/tempconvert.asmx</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>50</defaultValue>
      <description></description>
      <id>60f51238-c9ac-408e-a290-df0937bb9c7f</id>
      <masked>false</masked>
      <name>FarenheitInput</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'FahrenheitToCelsiusResponse.FahrenheitToCelsiusResult', '10')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
