<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT-update</name>
   <tag></tag>
   <elementGuidId>0fa52352-e6c7-4ee1-8bee-a894dfdd0eec</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;morpheus\&quot;,\n    \&quot;job\&quot;: \&quot;zion resident\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0b88434e-fd72-4c1b-b4c4-437c46a0cc0f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://reqres.in/api/users/${GlobalVariable.id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>326dd527-3027-41ba-8f98-e8c0b63e2086</id>
      <masked>false</masked>
      <name>variable</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ed1ad7a4-fa66-4f7f-885a-3b2cb5a2bf65</id>
      <masked>false</masked>
      <name>variable_0</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>437c6cb8-8236-4dfa-b109-5a882536c5db</id>
      <masked>false</masked>
      <name>variable_1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>88de3115-540b-459b-91a1-d14634372179</id>
      <masked>false</masked>
      <name>variable_2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>216b08d9-8a63-4ea0-98f1-4090100d3130</id>
      <masked>false</masked>
      <name>variable_3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2e98a5b1-0591-44af-8ba2-8673a594c27b</id>
      <masked>false</masked>
      <name>variable_4</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f5547276-5929-40ed-89ec-de980fd600c0</id>
      <masked>false</masked>
      <name>variable_5</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
