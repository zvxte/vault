document.addEventListener("DOMContentLoaded", () => {
/* --- */

// CONSTANTS
const http = window.__TAURI__.http;

const tabBtns = document.querySelectorAll(".tab-btn");
const passwordsTabBtn = document.querySelector(".passwords-tab-btn");
const notesTabBtn = document.querySelector(".notes-tab-btn");

let Entries = document.querySelectorAll(".entries");
let passwordEntries = document.querySelector(".password-entries");
let noteEntries = document.querySelector(".note-entries");
let passwordEntryDivs = document.querySelectorAll(".password-entry");
let noteEntryDivs = document.querySelectorAll(".note-entry");

const newEntryBtn = document.querySelector(".new-entry-btn");

const Editors = document.querySelectorAll(".editor");
const newPasswordEditor = document.querySelector(".new-password-editor");
const newNoteEditor = document.querySelector(".new-note-editor");
const editorCancelBtns = document.querySelectorAll(".editor-cancel-btn");
const editorSaveBtns = document.querySelectorAll(".editor-save-btn");

const passwordEditor = document.querySelector(".password-editor");
const passwordEditorId = document.querySelector(".password-editor-id");
const passwordEditorDomain = document.querySelector(".password-editor-domain");
const passwordEditorUsername = document.querySelector(".password-editor-username");
const passwordEditorPassword = document.querySelector(".password-editor-password");

const noteEditor = document.querySelector(".note-editor");
const noteEditorId = document.querySelector(".note-editor-id");
const noteEditorTitle = document.querySelector(".note-editor-title");
const noteEditorContent = document.querySelector(".note-editor-content");

// USER
const user = {
    // user_id: "",
    // username: "",
    // salt: [],
    session_id: ""
}

// STATES
let entriesState = null;
let editorState = null;

selectTab(passwordsTabBtn);
selectEntries(passwordEntries);
getAllPasswords();
addEventListeners();

// EVENT LISTENERS
function addEventListeners() {
    tabBtns.forEach(tabBtn => {
        tabBtn.addEventListener("click", (event) => {
            if (tabBtn === event.target) {
                selectTab(tabBtn);
                if (tabBtn === passwordsTabBtn) selectEntries(passwordEntries);
                else if (tabBtn === notesTabBtn) selectEntries(noteEntries);
            }
        })
    });

    newEntryBtn.addEventListener("click", () => {
        emptyEditor();
        if (entriesState === passwordEntries) selectNewEditor(newPasswordEditor);
        else if (entriesState === noteEntries) selectNewEditor(newNoteEditor);
    })

    passwordEntryDivs.forEach(passwordEntry => {
        passwordEntry.addEventListener("click", () => {
            selectEntry(passwordEntry);
            let passwordId = passwordEntry.dataset.passwordId;
            let domain = passwordEntry.dataset.domainName;
            let username = passwordEntry.dataset.username;
            let password = passwordEntry.dataset.password;
            editPasswordEntry(passwordId, domain, username, password);
        })
    });

    noteEntryDivs.forEach(noteEntry => {
        noteEntry.addEventListener("click", () => {
            selectEntry(noteEntry);
            let noteId = noteEntry.dataset.noteId;
            let title = noteEntry.dataset.title;
            let content = noteEntry.dataset.content;
            editNoteEntry(noteId, title, content);
        })
    })

    editorCancelBtns.forEach(cancelBtn => {
        cancelBtn.addEventListener("click", () => {
            emptyEditor();
        })
    });

    editorSaveBtns.forEach(saveBtn => {
        saveBtn.addEventListener("click", () => {
            save(editorState);
        })
    });
}

// TABS
function selectTab(selectedTabBtn) {
    tabBtns.forEach(tabBtn => {
        tabBtn.style.borderLeft = "3px solid transparent";
    });
    selectedTabBtn.style.borderLeft = "3px solid var(--main-accent)";
}

// ENTRIES
function selectEntries(selectedEntries) {
    Entries.forEach(entries => {
        entries.classList.add("d-none");
    });
    selectedEntries.classList.remove("d-none");
    entriesState = selectedEntries;
}

function selectEntry(entry) {
    passwordEntryDivs.forEach(passwordEntry => {
        passwordEntry.style.borderLeft = "3px solid transparent";
    });
    noteEntryDivs.forEach(noteEntry => {
        noteEntry.style.borderLeft = "3px solid transparent";
    });
    entry.style.borderLeft = "3px solid var(--main-accent)";
}

function editPasswordEntry(passwordId, domain, username, password) {
    passwordEditorId.dataset.passwordId = passwordId;
    passwordEditorDomain.value = domain;
    passwordEditorUsername.value = username;
    passwordEditorPassword.value = password;

    selectNewEditor(passwordEditor);
};

function editNoteEntry(noteId, title, content) {
    noteEditorId.dataset.noteId = noteId;
    noteEditorTitle.value = title;
    noteEditorContent.value = content;

    selectNewEditor(noteEditor);
}

function addPasswordEntry(passwordId, domainName, username, password) {

    const dLabel = document.createElement("label");
    dLabel.classList.add("entry-label");
    dLabel.innerText = "domain";
    const dSpan = document.createElement("span");
    dSpan.classList.add("entry-value");
    dSpan.innerText = domainName;
    const domainPair = document.createElement("div");
    domainPair.classList.add("entry-pair-row");
    domainPair.appendChild(dLabel);
    domainPair.appendChild(dSpan);

    const uLabel = document.createElement("label");
    uLabel.classList.add("entry-label");
    uLabel.innerText = "username";
    const uSpan = document.createElement("span");
    uSpan.classList.add("entry-value");
    uSpan.innerText = username;
    const usernamePair = document.createElement("div");
    usernamePair.classList.add("entry-pair-row");
    usernamePair.appendChild(uLabel);
    usernamePair.appendChild(uSpan);
    
    const pE = document.createElement("div");
    pE.classList.add("entry", "password-entry");
    pE.dataset.passwordId = passwordId;
    pE.dataset.domainName = domainName;
    pE.dataset.username = username;
    pE.dataset.password = password;
    pE.appendChild(domainPair);
    pE.appendChild(usernamePair);

    passwordEntries.appendChild(pE);

    Entries = document.querySelectorAll(".entries");
    passwordEntries = document.querySelector(".password-entries");
    passwordEntryDivs = document.querySelectorAll(".password-entry");

    addEventListeners();
}

// EDITOR
function selectNewEditor(newEditor) {
    Editors.forEach(editor => {
        editor.classList.add("d-none");
    });
    newEditor.classList.remove("d-none");
    editorState = newEditor;
}

function emptyEditor() {
    Editors.forEach(editor => {
        editor.classList.add("d-none");
    });
    passwordEntryDivs.forEach(passwordEntry => {
        passwordEntry.style.borderLeft = "3px solid transparent";
    });
    noteEntryDivs.forEach(noteEntry => {
        noteEntry.style.borderLeft = "3px solid transparent";
    });
}

async function save(editor) {
    const client = await http.getClient();
    switch (editor) {
        case newPasswordEditor:

        case passwordEditor:
            
        case newNoteEditor:

        case noteEditor:

    }
}

// HTTP
async function getAllPasswords() {
    const client = await http.getClient();
    const response = await client.request({
        method: "GET",
        url: "",
        headers: {session_id: user.session_id}
    })
        .then((response) => {
            response.data.forEach(password => {
                addPasswordEntry(password.password_id, password.domain_name, password.username, password.password);
            });
        })
}

/* --- */
})
