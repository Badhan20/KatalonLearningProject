<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Number to Words Conversion</name>
   <tag></tag>
   <elementGuidId>b7fe0811-7264-4eea-86ea-e1f875a51b4b</elementGuidId>
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
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;utf-8&quot;?>
&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
  &lt;soap:Body>
    &lt;NumberToWords xmlns=&quot;http://www.dataaccess.com/webservicesserver/&quot;>
      &lt;ubiNum>${NumberForWord}&lt;/ubiNum>
    &lt;/NumberToWords>
  &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.dataaccess.com/webservicesserver/NumberConversion.wso</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>20</defaultValue>
      <description></description>
      <id>8ffc56f9-0976-4610-b9bd-b2700c7a8d98</id>
      <masked>false</masked>
      <name>NumberForWord</name>
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

WS.verifyElementText(response, 'NumberToWordsResponse.NumberToWordsResult', 'twenty ')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
