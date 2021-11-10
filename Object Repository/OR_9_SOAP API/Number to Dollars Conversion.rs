<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Number to Dollars Conversion</name>
   <tag></tag>
   <elementGuidId>f4338155-e638-4ecd-a934-903b3aec4b76</elementGuidId>
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
    &lt;NumberToDollars xmlns=&quot;http://www.dataaccess.com/webservicesserver/&quot;>
      &lt;dNum>${NumberForDollar}&lt;/dNum>
    &lt;/NumberToDollars>
  &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.dataaccess.com/webservicesserver/NumberConversion.wso</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>10</defaultValue>
      <description></description>
      <id>78c98065-643c-4f2d-af35-819d927d12ef</id>
      <masked>false</masked>
      <name>NumberForDollar</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

WS.verifyElementText(response, 'NumberToDollarsResponse.NumberToDollarsResult', 'ten dollars')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
