
package com.minosiants.two.contacts

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.ContentResolver
import android.database.Cursor
import android.provider.ContactsContract
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin



@TauriPlugin(
    permissions = [
        Permission(strings = [Manifest.permission.READ_CONTACTS], alias = "readContacts")
    ]
)
class ContactsPlugin(private val activity: Activity) : Plugin(activity) {
    @Command
    fun getContacts(invoke: Invoke) {
        invoke.resolveObject(Query(activity.contentResolver).q())
    }

}


class Query(private val contentResolver: ContentResolver) {

    @SuppressLint("Range")
    fun  q():List<Contact>  {
        Log.d("Query:", "getContacts")
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


data class PermissionStatus(val contacts:String)

data class Contact(val id: String, val name: String, val phone:String)

