# Authentication

## Options

- Password
- [fido2](https://en.wikipedia.org/wiki/FIDO2_Project) through Client to Authenticator Protocol ([CTAP](https://en.wikipedia.org/wiki/Client_to_Authenticator_Protocol))

## How
 We therefore act as the gatekeeper of the user's data, and must also be aware of their key. This means that we are responsible for keeping the user's key secure. 

Flow looks something like this:

1. User provides email and desired password in registration.
   - A choice must be made at this step, as the user's password is in plain text when typed into the UI.
      - Hash the user's password on the UI/Client side before passing to server
      - Use SSL to ensure 
2. The password is passed from the client to the backend and the password is immediately hashed
   - The password is hashed 
