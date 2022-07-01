import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.logging.KeywordLogger as KeywordLogger
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.util.CryptoUtil as CryptoUtil
import internal.GlobalVariable as GlobalVariable
import static org.assertj.core.api.Assertions.*
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

def slurper = new JsonSlurper()

def requestPutUpdate = WS.sendRequest(findTestObject('Object Repository/PUT-update'))

KeywordUtil.logInfo('RESPONSE BODY')

KeywordUtil.logInfo(requestPutUpdate.getResponseBodyContent())

def responsePutUpdate = slurper.parseText(requestPutUpdate.getResponseBodyContent())

KeywordUtil.logInfo('VERIFICATION VALUE START')

//verify request stastus code
WS.verifyResponseStatusCode(requestPutUpdate, 200)

KeywordUtil.logInfo('VERIFICATION VALUE DONE')
