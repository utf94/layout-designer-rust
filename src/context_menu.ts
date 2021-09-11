export class ContextMenu extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    this.style.display = "none";
  }

  open(event, content) {
    this.style.display = "block";
    this.style.left = event.x + "px";
    this.style.top = event.y + "px";

    this.innerHTML = "";
    this.appendChild(content);
  }

  close() {
    this.style.display = "none";
  }
}

customElements.define("context-menu", ContextMenu);

export function connect() {
  document.addEventListener(
    "contextmenu",
    (event) => {
      event.preventDefault();

      // @ts-ignore
      if (event.target.open_contextmenu) {
        // @ts-ignore
        event.target.open_contextmenu(event);
      }
    },
    false
  );

  document.addEventListener("click", (event) => {
    const context_menu = document.querySelector("context-menu");

    // @ts-ignore
    if (context_menu.contains(event.target)) {
      //
    } else {
      // @ts-ignore
      context_menu.close();
    }
  });
}
