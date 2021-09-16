import { Editor } from "./index";

export function connect(editor: Editor) {
  document
    .getElementById("page-width-input")
    .addEventListener("input", (event) => {
      const target = event.target as HTMLInputElement;

      let page = document.querySelector(".page") as HTMLElement;
      if (page) {
        editor.resize_page(page, parseInt(target.value));
      }
    });

  document
    .getElementById("page-fill-input")
    .addEventListener("input", (event) => {
      const target = event.target as HTMLInputElement;

      const page = document.querySelector("page") as HTMLElement;
      page.style.backgroundColor = target.value;

      let text = target.value.split("#")[1];
      document.getElementById("page-fill-span").innerText = text.toUpperCase();
    });
}
