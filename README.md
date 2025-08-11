# Two

## Tauri

- create a new project
  ```
  pnpm create tauri-app
  cd test-app
  pnpm install
  pnpm tauri android init
  pnpm tauri ios init
  ```

## Plugins

- create a new plugin
  `pnpm tauri plugin new --directory /Users/kaspar/stuff/sources/two/src-tauri/plugins/contacts  contacts`

### ios sims

~/Library/Developer/CoreSimulator/Devices

find . -name `*two*`

### android

Yes, your understanding is exactly correct. 👍

You must first declare the permission your app might need in the AndroidManifest.xml file, and then you use the requestPermissions() method at runtime to actually ask the user for their consent.

Think of it as a two-step process required for "dangerous" permissions (like accessing contacts, location, camera, etc.).

The Two-Step Permission Process

1. Declaration in the Manifest
   This is the static declaration to the Android system and the Google Play Store. By adding <uses-permission android:name="android.permission.READ_CONTACTS" /> to your manifest, you are telling the system: "My app includes features that might require the ability to read the user's contacts."

Purpose: This provides transparency. Before the user even runs the app, the system knows its potential capabilities. It's a necessary prerequisite.

2. Request at Runtime
   This is the dynamic request made to the user while they are using your app. The requestPermissions() method triggers a system dialog box that explicitly asks the user for their consent (e.g., "Allow [Your App] to access your contacts?").

Purpose: This gives the user control and context. You should only call this method right before you need the permission. For example, when the user taps a button to "Import contacts," it's the perfect time to ask. This way, the user understands why your app needs that specific permission at that moment.

In short, the manifest declaration is like telling the government you might want to drive a car someday, while the runtime request is like going to the DMV to get your actual driver's license before you get behind the wheel. You must do both.
