package com.minosiants.two.contacts
import android.annotation.SuppressLint
import android.content.ContentResolver
import android.database.Cursor
import android.provider.ContactsContract
import android.util.Log


class Query(private val contentResolver: ContentResolver) {

    @SuppressLint("Range")
    fun  q():List<Contact>  {
        val cursor = contentResolver.query(
            ContactsContract.Contacts.CONTENT_URI,
            null, null, null, null
        )
        val result = mutableListOf<Contact>()
        cursor?.use { c: Cursor ->
            Log.d("Query", "Columns: " + c.columnNames.joinToString())
            while (c.moveToNext()) {
                val id = c.getString(c.getColumnIndexOrThrow(ContactsContract.Contacts._ID))
                val name = c.getString(c.getColumnIndexOrThrow(ContactsContract.Contacts.DISPLAY_NAME))
                if (id != null && name != null) {
                    result.add(Contact(id, name, "num"))
                }
            }
        }
        return result
    }
}


data class Contact(val id: String, val name: String, val phone:String)

