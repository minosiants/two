package com.minosiants.two.contacts
import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin


@TauriPlugin
class ContactsPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
     fun contacts(invoke: Invoke) {

        if (ContextCompat.checkSelfPermission(
                activity,
                Manifest.permission.READ_CONTACTS
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            ActivityCompat.requestPermissions(activity, arrayOf(Manifest.permission.READ_CONTACTS), 123)
        }

        invoke.resolveObject(Query(activity.contentResolver).q())
    }

}
