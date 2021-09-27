import { LayoutContainer } from "./container";
import { Editor } from "./index";

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

export function connect(editor: Editor) {
  document.addEventListener(
    "contextmenu",
    (event) => {
      event.preventDefault();

      const target = event.target as LayoutContainer;
      if (target.open_contextmenu) {
        target.open_contextmenu(editor, event);
      }
    },
    false
  );

  document.addEventListener("click", (event) => {
    const context_menu = document.querySelector("context-menu") as ContextMenu;

    if (context_menu.contains(event.target as Node)) {
      //
    } else {
      context_menu.close();
    }
  });
}
