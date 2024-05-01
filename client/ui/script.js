document.addEventListener("DOMContentLoaded", () => {
/* --- */

// STRUCTURES
class User {
    constructor(user_id, username, plain_master_password, salt) {
        this.user_id = user_id,
        this.username = username,
        this.plain_master_password = plain_master_password,
        this.salt = salt  // byte array
    }
}

class Session {
    constructor(session_id) {
        this.session_id = session_id
    }
}

class Password {
    constructor(password_id, domain_name, username, password, nonce) {
        this.password_id = password_id;
        this.domain_name = domain_name;
        this.username = username;
        this.password = password;
        this.nonce = nonce;
    }

    static from(data) {
        return new Password(
            data.password_id, data.domain_name, data.username, data.password, data.nonce
        )
    }

    static async encryptPassword(password) {
        const result = await invoke("encrypt", {
            data: password
        });
        console.log("result: ", result);
        return result
    }

    static async decryptPassword(password, nonce) {
        const result = await invoke("decrypt", {
            data: password,
            nonce: nonce
        })
        return result
    }
}

class Server {
    constructor(address) {
        this.address = address;  // e.g. 127.0.0.1:5000
        this.http = window.__TAURI__.http;
    }

    async initializeClient() {
        this.client = await this.http.getClient();
    }

    async postUsersRegister(username, password) {
        const response = await this.client.request({
            method: "POST",
            url: this.address + "/users/register",
            headers: {
                ContentType: "application/json"
            },
            body: this.http.Body.json({
                username: username,
                password: password
            }),
            responseType: this.http.ResponseType.JSON
        });
        return response
    }

    async postUsersLogin(username, password) {
        const response = await this.client.request({
            method: "POST",
            url: this.address + "/users/login",
            headers: {
                ContentType: "application/json"
            },
            body: this.http.Body.json({
                username: username,
                password: password
            }),
            responseType: this.http.ResponseType.JSON
        });
        return response
    }

    async postUsersLogout(session_id) {
        const response = await this.client.request({
            method: "POST",
            url: this.address + "/users/logout",
            headers: {
                session_id: session_id
            },
            responseType: this.http.ResponseType.JSON
        });
        return response;
    }

    async postPasswords(session_id, domain_name, username, password, nonce) {
        const response = await this.client.request({
            method: "POST",
            url: this.address + "/passwords",
            headers: {
                ContentType: "application/json",
                session_id: session_id
            },
            body: this.http.Body.json({
                domain_name: domain_name,
                username: username,
                password: password,
                nonce: nonce
            }),
            responseType: this.http.ResponseType.JSON
        });
        return response
    }

    async getPasswordsId(session_id, password_id) {
        const response = await this.client.request({
            method: "GET",
            url: this.address + "/passwords/" + password_id,
            headers: {
                session_id: session_id
            },
            responseType: this.http.ResponseType.JSON
        });
        return response;
    }

    async getPasswords(session_id) {
        const response = await this.client.request({
            method: "GET",
            url: this.address + "/passwords",
            headers: {
                session_id: session_id
            },
            responseType: this.http.ResponseType.JSON
        });
        return response;
    }

    async deletePasswordsId(session_id, password_id) {
        const response = await this.client.request({
            method: "DELETE",
            url: this.address + "/passwords/" + password_id,
            headers: {
                session_id: session_id
            },
            responseType: this.http.ResponseType.JSON
        });
        return response;
    }

    async patchPasswordsId(session_id, password_id, domain_name, username, password, nonce) {
        const response = await this.client.request({
            method: "PATCH",
            url: this.address + "/passwords/" + password_id,
            headers: {
                ContentType: "application/json",
                session_id: session_id
            },
            body: this.http.Body.json({
                domain_name: domain_name,
                username: username,
                password: password,
                nonce: nonce
            }),
            responseType: this.http.ResponseType.JSON
        });
        return response;
    }
}

class App  {
    async main() {
        await this.setup();
    }

    async setup() {
        // account buttons
        const loginBtn = document.querySelector(".login-btn");
        loginBtn.addEventListener("click", () => {
            const loginEditor = document.querySelector(".login-editor");
            this.selectEditor(loginEditor);
        });

        const registerBtn = document.querySelector(".register-btn");
        registerBtn.addEventListener("click", () => {
            const registerEditor = document.querySelector(".register-editor");
            this.selectEditor(registerEditor);
        });

        // tabs
        const passwordsTabBtn = document.querySelector(".passwords-tab-btn");
        passwordsTabBtn.addEventListener("click", () => {
            const passwordEntries = document.querySelector(".password-entries");
            this.selectEntries(passwordEntries, passwordsTabBtn);
        });

        const notesTabBtn = document.querySelector(".notes-tab-btn");
        notesTabBtn.addEventListener("click", () => {
            const noteEntries = document.querySelector(".note-entries");
            this.selectEntries(noteEntries, notesTabBtn);
        });

        // editor buttons
        const cancelBtns = document.querySelectorAll(".editor-cancel-btn");
        cancelBtns.forEach(cancelBtn => {
            cancelBtn.addEventListener("click", () => {
                this.unselectEditor();
                this.unselectEntry();
            })
        });

        const editorRegisterBtn = document.querySelector(".editor-register-btn");
        editorRegisterBtn.addEventListener("click", async () => {
            const editorRegisterUsername = document.querySelector(".editor-register-username");
            const editorRegisterPassword = document.querySelector(".editor-register-password");
            const editorRegisterServerAddress = document.querySelector(".editor-register-server-address");
            this.server = new Server(editorRegisterServerAddress.value);
            await this.server.initializeClient();
            await this.server.postUsersRegister(
                editorRegisterUsername.value, editorRegisterPassword.value
            )
                .then((result) => {
                    if (result.ok == false) this.showFailureNotification(result.data.message)
                    else this.showSuccessNotification(result.data.message)
                })
                .catch((error) => {
                    this.showFailureNotification(error);
                })
        });

        const editorLoginBtn = document.querySelector(".editor-login-btn");
        editorLoginBtn.addEventListener("click", async () => {
            const editorLoginUsername = document.querySelector(".editor-login-username");
            const editorLoginPassword = document.querySelector(".editor-login-password");
            const editorLoginServerAddress = document.querySelector(".editor-login-server-address");
            this.server = new Server(editorLoginServerAddress.value);
            await this.server.initializeClient();
            await this.server.postUsersLogin(
                editorLoginUsername.value, editorLoginPassword.value
            )
                .then((result) => {
                    if (result.ok == false) this.showFailureNotification(result.data.message)
                    else {
                        this.showSuccessNotification("Logged in");
                        this.user = new User(result.data.user_id, result.data.username, editorLoginPassword.value, result.data.salt);
                        this.session = new Session(result.headers.session_id);
                        this.unselectEditor();
                        this.setupAfterLogin();
                    }
                })
                .catch((error) => {
                    this.showFailureNotification(error);
                })
        })
        const loginEditor = document.querySelector(".login-editor");
        this.selectEditor(loginEditor);
    }

    async setupAfterLogin() {
        document.querySelector(".login-btn").classList.add("d-none");
        document.querySelector(".register-btn").classList.add("d-none");
        const logoutBtn = document.querySelector(".logout-btn");
        logoutBtn.classList.remove("d-none");
        logoutBtn.addEventListener("click", async () => {
            await this.server.postUsersLogout(this.session.session_id)
                .then((result) => {
                    if (result.ok == false) this.showFailureNotification(result.data.message)
                    else {
                        this.showSuccessNotification(result.data.message);
                        this.unselectEditor();
                        this.unselectEntry();
                        this.deleteEntries();
                        this.session = null;
                        this.server = null;
                        this.user = null;
                        logoutBtn.classList.add("d-none");
                        document.querySelector(".login-btn").classList.remove("d-none");
                        document.querySelector(".register-btn").classList.remove("d-none");
                    }
                })
                .catch((error) => {
                    this.showFailureNotification(error)
                })
        })

        const passwordsTabBtn = document.querySelector(".passwords-tab-btn");
        const passwordEntries = document.querySelector(".password-entries");
        this.selectEntries(passwordEntries, passwordsTabBtn);

        // initialize encrypter
        await invoke("create_encrypter", {
            plainMasterPassword: this.user.plain_master_password,
            salt: this.user.salt
        });

        // enable `NEW` buttons
        const newPasswordBtn = document.querySelector(".new-password-btn");
        newPasswordBtn.addEventListener("click", () => {
            this.unselectEntry();
            const passwordEditor = document.querySelector(".new-password-editor");
            this.selectEditor(passwordEditor);
        })

        const newNoteBtn = document.querySelector(".new-note-btn");
        newNoteBtn.addEventListener("click", () => {
            this.unselectEntry();
            const noteEditor = document.querySelector(".new-note-editor");
            this.selectEditor(noteEditor);
        })

        // load all existing passwords
        const passwords = [];
        await this.server.getPasswords(this.session.session_id)
            .then((result) => {
                if (result.ok == false) this.showFailureNotification(result.data.message)
                else result.data.forEach(password => {
                    passwords.push(Password.from(password))
                });
            })
            .catch((error) => {
                this.showFailureNotification(error);
            })
        passwords.forEach(password => {
            console.log(password);
            this.addPasswordEntry(password);
        });

        // load all existing notes (todo)

        // enable editor buttons (todo for notes)
        const newPasswordEditorSaveBtn = document.querySelector(".new-password-editor-save-btn");
        newPasswordEditorSaveBtn.addEventListener("click", async () => {
            const domainName = document.querySelector(".new-password-editor-domain").value;
            const username = document.querySelector(".new-password-editor-username").value;
            const password = document.querySelector(".new-password-editor-password").value;

            let encryptedData = await Password.encryptPassword(password);

            await this.server.postPasswords(this.session.session_id, domainName, username, encryptedData[0], encryptedData[1])
                .then((result) => {
                    const password = Password.from(result.data);
                    this.addPasswordEntry(password);
                    this.unselectEditor();
                    this.showSuccessNotification("Password created");
                })
                .catch((error) => { this.showFailureNotification(error) })
        });

        const passwordEditorSaveBtn = document.querySelector(".password-editor-save-btn");
        passwordEditorSaveBtn.addEventListener("click", async () => {
            const passwordId = document.querySelector(".password-editor-id").dataset.passwordId;
            const domainName = document.querySelector(".password-editor-domain").value;
            const username = document.querySelector(".password-editor-username").value;
            const password = document.querySelector(".password-editor-password").value;

            let encryptedData = await Password.encryptPassword(password);
            console.log(passwordId, domainName, username, encryptedData[0], encryptedData[1]);
            await this.server.patchPasswordsId(this.session.session_id, passwordId, domainName, username, encryptedData[0], encryptedData[1])
                .then((result) => {
                    if (result.ok == false) {this.showFailureNotification("Failed to update password")}
                    else {
                        console.log(result);
                        this.deletePasswordEntry(passwordId);
                        const password = Password.from(result.data);
                        this.addPasswordEntry(password);
                        this.unselectEditor();
                        this.showSuccessNotification("Password updated");
                    }
                })
                .catch((error) => { this.showFailureNotification(error) })
        });

        const passwordEditorDeleteBtn = document.querySelector(".password-editor-delete-btn");
        passwordEditorDeleteBtn.addEventListener("click", async () => {
            const passwordId = document.querySelector(".password-editor-id").dataset.passwordId;

            await this.server.deletePasswordsId(this.session.session_id, passwordId)
                .then((_result) => {
                    this.deletePasswordEntry(passwordId);
                    this.unselectEditor();
                    this.unselectEntry();
                    this.showSuccessNotification("Password deleted");
                })
                .catch((error) => { this.showFailureNotification(error) })
        });
    }
    
    unselectEditor() {
        const editors = document.querySelectorAll(".editor");
        editors.forEach(editor => {
            editor.classList.add("d-none");
        });
    }

    selectEditor(selectedEditor) {
        this.unselectEditor();
        selectedEditor.classList.remove("d-none");
    }

    fillPasswordEditor(passwordEntry) {
        const idInput = document.querySelector(".password-editor-id");
        const domainInput = document.querySelector(".password-editor-domain");
        const usernameInput = document.querySelector(".password-editor-username");
        const passwordInput = document.querySelector(".password-editor-password");
        idInput.dataset.passwordId = passwordEntry.dataset.passwordId;
        domainInput.value = passwordEntry.dataset.domainName;
        usernameInput.value = passwordEntry.dataset.username;
        passwordInput.value = passwordEntry.dataset.password;
    }

    clearPasswordEditor() {
        const idInput = document.querySelector(".password-editor-id");
        const domainInput = document.querySelector(".password-editor-domain");
        const usernameInput = document.querySelector(".password-editor-username");
        const passwordInput = document.querySelector(".password-editor-password");
        idInput.dataset.passwordId = "";
        domainInput.value = "";
        usernameInput.value = "";
        passwordInput.value = "";
    }

    selectEntries(selectedEntries, selectedTabBtn) {
        const entries = document.querySelectorAll(".entries");
        entries.forEach(entry => {
            entry.classList.add("d-none");
        });
        selectedEntries.classList.remove("d-none");
        
        const tabBtns = document.querySelectorAll(".tab-btn");
        tabBtns.forEach(tabBtn => {
            tabBtn.classList.remove("tab-btn-selected");
        });
        selectedTabBtn.classList.add("tab-btn-selected");
    }

    unselectEntry() {
        const allEntries = document.querySelectorAll(".entry");
        allEntries.forEach(entry => {
            entry.classList.remove("entry-selected");
        })
    }

    selectEntry(selectedEntry) {
        this.unselectEntry();
        selectedEntry.classList.add("entry-selected");
    }

    showFailureNotification(message, timeout = 3000) {
        const failureNotification = document.querySelector(".failure-notification");
        const failureNotificationValue = document.querySelector(".failure-notification-value");
        failureNotificationValue.innerText = message;
        failureNotification.classList.remove("d-none");
        setTimeout(() => {
            failureNotification.classList.add("d-none");
            failureNotificationValue.value = "";
        }, timeout);
    }

    showSuccessNotification(message, timeout = 3000) {
        const successNotification = document.querySelector(".success-notification");
        const successNotificationValue = document.querySelector(".success-notification-value");
        successNotificationValue.innerText = message;
        successNotification.classList.remove("d-none");
        setTimeout(() => {
            successNotification.classList.add("d-none");
            successNotificationValue.value = "";
        }, timeout);
    }

    async addPasswordEntry(password) {
        const domainLabel = document.createElement("label");
        domainLabel.classList.add("entry-label");
        domainLabel.innerText = "domain";
        const domainValue = document.createElement("span");
        domainValue.classList.add("entry-value");
        domainValue.innerText = password.domain_name;
        const domainPair = document.createElement("div");
        domainPair.classList.add("entry-pair-row");
        domainPair.appendChild(domainLabel);
        domainPair.appendChild(domainValue);

        const usernameLabel = document.createElement("label");
        usernameLabel.classList.add("entry-label");
        usernameLabel.innerText = "username";
        const usernameValue = document.createElement("span");
        usernameValue.classList.add("entry-value");
        usernameValue.innerText = password.username;
        const usernamePair = document.createElement("div");
        usernamePair.classList.add("entry-pair-row");
        usernamePair.appendChild(usernameLabel);
        usernamePair.appendChild(usernameValue);
        
        const decryptedPassword = await Password.decryptPassword(password.password, password.nonce)
            .catch((error) => {
                this.showFailureNotification(error);
            })

        const passwordEntry = document.createElement("div");
        passwordEntry.classList.add("entry", "password-entry");
        passwordEntry.dataset.passwordId = password.password_id;
        passwordEntry.dataset.domainName = password.domain_name;
        passwordEntry.dataset.username = password.username;
        passwordEntry.dataset.password = decryptedPassword;
        passwordEntry.appendChild(domainPair);
        passwordEntry.appendChild(usernamePair);

        passwordEntry.addEventListener("click", () => {
            this.selectEntry(passwordEntry);
            const passwordEditor = document.querySelector(".password-editor");
            this.selectEditor(passwordEditor);
            this.fillPasswordEditor(passwordEntry);
        });

        const passwordEntries = document.querySelector(".password-entries");
        passwordEntries.appendChild(passwordEntry);

        this.unselectEditor();
        this.clearPasswordEditor();
    }

    deletePasswordEntry(passwordId) {
        const passwordEntries = document.querySelectorAll(".password-entry");
        passwordEntries.forEach(passwordEntry => {
            if (passwordEntry.dataset.passwordId === passwordId) {
                passwordEntry.remove();
            }
        });
    }

    deleteEntries() {
        const entries = document.querySelectorAll(".entry");
        entries.forEach(entry => {
            entry.remove();
        });
    }
}

const invoke = window.__TAURI__.invoke;
const app = new App();
app.main();

/* --- */
})
