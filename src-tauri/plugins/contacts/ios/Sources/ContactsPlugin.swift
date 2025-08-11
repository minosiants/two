import Tauri
import Contacts
import UIKit



struct Contact : Encodable {
   let id:String 
   let name:String
   let phone:String


        
} 

class ContactsPlugin :Plugin{
    private let store = CNContactStore()

     @objc public override func checkPermissions(_ invoke:Invoke){
        let status = CNContactStore.authorizationStatus(for: .contacts)

        invoke.resolve(["readContacts": toPermissionState(status)])

    }
    
    @objc public override func requestPermissions(_ invoke:Invoke) {
        let status = CNContactStore.authorizationStatus(for: .contacts)

        store.requestAccess(for: .contacts) { granted, error in
            if let error = error {
                invoke.reject(error.localizedDescription)
                return
            }
            
            let permissionState = granted ? "granted" : "denied"
            invoke.resolve(["readContacts": permissionState])
        }
    }


    @objc public func getContacts(_ invoke: Invoke) throws {

            guard CNContactStore.authorizationStatus(for: .contacts) == .authorized else {
                invoke.reject("Contacts access denied")
                return
            }

            var keysToFetch = defaultKeysToFetch()

            let request = CNContactFetchRequest(keysToFetch: keysToFetch)



            var contacts: [Contact] = []
            var count = 0

            do {
                try store.enumerateContacts(with: request) { contact, stop in
                    contacts.append(self.toContact(contact))
                    count += 1

                }
                invoke.resolve(contacts)
            } catch {
                invoke.reject("Failed to fetch contacts: \(error.localizedDescription)")
            }
        }


    private func toPermissionState(_ status: CNAuthorizationStatus) -> String {
        switch status {
        case .notDetermined:
            return "prompt"
        case .restricted, .denied:
            return "denied"
        case .authorized:
            return "granted"
        @unknown default:
            // This handles .limited on iOS 18+ and any future cases
            if #available(iOS 18.0, *) {
                // Check if it's the limited case
                return "limited"
            }
            return "denied"
        }
    }

     private func defaultKeysToFetch() -> [CNKeyDescriptor] {
            return [
                CNContactIdentifierKey,
                CNContactGivenNameKey,
                CNContactFamilyNameKey,
                CNContactMiddleNameKey,
                CNContactPhoneNumbersKey
            ] as [CNKeyDescriptor]
     }
     private func toContact(_ contact: CNContact) -> Contact {
        var name = "\(contact.givenName ?? "") \(contact.familyName ?? "")"
        var phone = contact.phoneNumbers.first?.value.stringValue ?? ""  
            return Contact(id:contact.identifier, name:name, phone:phone)
         }
}


@_cdecl("init_plugin_contacts")
func initPlugin() -> Plugin {
    return ContactsPlugin()
}
