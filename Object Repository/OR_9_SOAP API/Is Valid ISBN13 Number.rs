<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Is Valid ISBN13 Number</name>
   <tag></tag>
   <elementGuidId>ef8b9543-09be-4a7f-b693-c82e3836c0a7</elementGuidId>
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
    &lt;IsValidISBN13 xmlns=&quot;http://webservices.daehosting.com/ISBN&quot;>
      &lt;sISBN>${ISBNNumber}&lt;/sISBN>
    &lt;/IsValidISBN13>
  &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://webservices.daehosting.com/services/isbnservice.wso</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'978-1-4612-9090-2'</defaultValue>
      <description></description>
      <id>aae8622a-b4f8-4af3-9967-c6e7249cd5a5</id>
      <masked>false</masked>
      <name>ISBNNumber</name>
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

WS.verifyElementText(response, 'IsValidISBN13Response.IsValidISBN13Result', 'true')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
