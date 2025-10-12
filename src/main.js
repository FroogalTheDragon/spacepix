const { invoke } = window.__TAURI__.core;
const { Menu } = window.__TAURI__.menu;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

const appMenu = await Menu.new({
  items: [
    { id: 'quit', text: 'Quit', action: () => console.log('quit pressed') },
    { id: 'check_item', text: 'Check Item', checked: true },
    { type: 'Separator' },
    { id: 'status', text: 'Status: Processing...' },
  ],
});

await appMenu.setAsAppMenu();

// expose helpers if you want to call from the page
window.appMenu = appMenu;
window.setMenuStatus = async (text) => {
  const statusItem = await appMenu.get('status');
  if (statusItem) await statusItem.setText(text);
};


window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  const greetForm = document.querySelector("#greet-form");
  if (greetForm) {
    greetForm.addEventListener("submit", (e) => {
      e.preventDefault();
      greet();
    });
  }

  // TODO: Figure out why navbar is not showing up
  // Inject a simple interactive navbar at the top of the page
  const nav = document.createElement('header');
  nav.id = 'navbar';
  nav.innerHTML = `
    <nav class="navbar">
      <button id="status-ready">Set Ready</button>
      <button id="status-processing">Set Processing</button>
    </nav>
  `;
  document.body.prepend(nav);

  document.getElementById('status-ready')?.addEventListener('click', async () => {
    await window.setMenuStatus?.('Status: Ready');
  });
  document.getElementById('status-processing')?.addEventListener('click', async () => {
    await window.setMenuStatus?.('Status: Processing...');
  });
});