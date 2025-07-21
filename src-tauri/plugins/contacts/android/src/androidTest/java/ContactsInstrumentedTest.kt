package com.minosiants.two.contacts


import android.content.ContentValues
import android.provider.ContactsContract
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Assert.*
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith


/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class ContactsInstrumentedTest {

    private lateinit var contentResolver: android.content.ContentResolver
@Before
fun grantPermissions() {
    InstrumentationRegistry.getInstrumentation().uiAutomation
        .executeShellCommand("pm grant com.minosiants.two.contacts android.permission.READ_CONTACTS")

    InstrumentationRegistry.getInstrumentation().uiAutomation
        .executeShellCommand("pm grant com.minosiants.two.contacts android.permission.WRITE_CONTACTS")
    val context = InstrumentationRegistry.getInstrumentation().targetContext
    contentResolver = context.contentResolver


}
    @Test
    fun testContactList() {
        // Insert RawContact
        val rawContactValues = ContentValues()
        val rawContactUri = contentResolver.insert(ContactsContract.RawContacts.CONTENT_URI, rawContactValues)
        val rawContactId = rawContactUri?.lastPathSegment

        // Insert Data (Display Name)
        val dataValues = ContentValues().apply {
            put(ContactsContract.Data.RAW_CONTACT_ID, rawContactId)
            put(ContactsContract.Data.MIMETYPE, ContactsContract.CommonDataKinds.StructuredName.CONTENT_ITEM_TYPE)
            put(ContactsContract.CommonDataKinds.StructuredName.DISPLAY_NAME, "John")
        }
        contentResolver.insert(ContactsContract.Data.CONTENT_URI, dataValues)

        val result = Query(contentResolver).q()
        val person = result.find { it.name.name == "John" }
        assertNotNull(person)
    }
}
