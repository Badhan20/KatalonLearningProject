import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

res_Cel_To_Faren = WS.sendRequest(findTestObject('OR_9_SOAP API/Celsius to Farenheit Conversion'))

WS.verifyResponseStatusCode(res_Cel_To_Faren, 200)

WS.verifyElementText(res_Cel_To_Faren, 'CelsiusToFahrenheitResponse.CelsiusToFahrenheitResult', CelsiusOutput)

res_Faren_To_Cel = WS.sendRequest(findTestObject('OR_9_SOAP API/Farenheit to Celsius Conversion'))

WS.verifyResponseStatusCode(res_Faren_To_Cel, 200)

WS.verifyElementText(res_Faren_To_Cel, 'FahrenheitToCelsiusResponse.FahrenheitToCelsiusResult', FarenheitOutput)

res_ISBN_Number = WS.sendRequest(findTestObject('OR_9_SOAP API/Is Valid ISBN13 Number'))

WS.verifyResponseStatusCode(res_ISBN_Number, 200)

WS.verifyElementText(res_ISBN_Number, 'IsValidISBN13Response.IsValidISBN13Result', ISBNNumberOutput)

res_Num_To_Dollar = WS.sendRequest(findTestObject('OR_9_SOAP API/Number to Dollars Conversion'))

WS.verifyResponseStatusCode(res_Num_To_Dollar, 200)

WS.verifyElementText(res_Num_To_Dollar, 'NumberToDollarsResponse.NumberToDollarsResult', NumberForDollarOutput)

res_Number_To_Word = WS.sendRequest(findTestObject('OR_9_SOAP API/Number to Words Conversion'))

WS.verifyResponseStatusCode(res_Number_To_Word, 200)

WS.verifyElementText(res_Number_To_Word, 'NumberToWordsResponse.NumberToWordsResult', NumberForWordOutput)

