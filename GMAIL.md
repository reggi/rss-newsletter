## Google Workspace Setup

> Enabling 2fa, getting a "App Password" and `emabling smtp` routing was all I needed to do for my workspace.

* Search "App Password" in google account https://myaccount.google.com/u/1/apppasswords (may need 2fa)
* "Google Workspace" -> "Admin" -> "Security" -> "Overview" -> "2-Step Verification"
  * Check `Allow users to turn on 2-Step Verification`
  * https://admin.google.com/u/2/ac/security/2sv
* Apps -> Google Workspace -> Settings for Gmail -> Routing -> SMTP relay service

Description | Status | Source | Values
-- | -- | -- | --
enable smtp | Enabled | Locally applied | Allowed Senders: Only addresses in my domains Only accept mail from the specified IP addresses: OFF Allowed IP addresses: Require SMTP Authentication: ON Require TLS encryption: OFF


## App Specific Passwords in Google Workspace not eligable? [Source](https://mycustomsoftware.com/app-specific-passwords-in-google-workspace-not-eligable/)

Why is my App-Specific Passwords feature disabled?

1. You need to disable the Less Secure apps feature
2. You must enable 2-step verification for your account
3. Then you will unlock the App-Specific Passwords feature

## How to enable App-Specific Passwords

Gmail accounts have an App-Specific Password feature instead of enabling the Less Secure apps feature for connecting 3rd party apps to your Gmail account.

1. Go to https://myaccount.google.com/ and Login
2. Click On Security
3. Enable 2-Step Verification under Signing into Google
4. Click App passwords (and sign in again)
5. On the App Passwords screen use the drop-downs to select the usage (it doesn’t really matter what you choose)
6. Click the GENERATE button.
7. Your new password will be randomly generated and displayed on your screen.
8. Save it somewhere safe as you only get to see this once and will need to generate a new one if you lose it.
9. Use this new generated password in place of your regular gmail password when signing in with a Less Secure App.
