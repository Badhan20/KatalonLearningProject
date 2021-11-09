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

//Open Browser
WebUI.openBrowser('http://lifecharger.org/')

WebUI.maximizeWindow()

//Click "Stretch Zone" section
WebUI.click(findTestObject('OR_4_Script Mode/Page_Life Charger   a new beginning/a_Stretch Zone  Why Nobody talks about this'))

//Search Text on Textbox
WebUI.setText(findTestObject('OR_4_Script Mode/Page_Life Charger   a new beginning/input_Search for_s'), 'life')

//Click on search button
WebUI.click(findTestObject('OR_4_Script Mode/Page_Life Charger   a new beginning/button_Search for_search-submit'))

//Click "Home" tab
WebUI.click(findTestObject('Object Repository/OR_4_Script Mode/Page_Life Charger   a new beginning/a_Home'))

//Close the browser
WebUI.closeBrowser()

