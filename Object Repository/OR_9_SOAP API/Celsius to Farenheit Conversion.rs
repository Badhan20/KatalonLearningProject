<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Celsius to Farenheit Conversion</name>
   <tag></tag>
   <elementGuidId>974050d1-3361-422a-b3aa-16e4caf58339</elementGuidId>
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
    &lt;CelsiusToFahrenheit xmlns=&quot;https://www.w3schools.com/xml/&quot;>
      &lt;Celsius>${CelsiusInput}&lt;/Celsius>
    &lt;/CelsiusToFahrenheit>
  &lt;/soap12:Body>
&lt;/soap12:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.w3schools.com/xml/tempconvert.asmx</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>20</defaultValue>
      <description></description>
      <id>67c58235-f88d-44af-9e54-544a88775a7f</id>
      <masked>false</masked>
      <name>CelsiusInput</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

WS.verifyElementText(response, 'CelsiusToFahrenheitResponse.CelsiusToFahrenheitResult', '68')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
