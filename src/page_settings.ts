import { Editor } from "./index";

function connect_width_input(editor: Editor) {
  const target = document.getElementById(
    "page-width-input"
  ) as HTMLInputElement;

  const minStr = target.getAttribute("min");
  const min = minStr ? parseInt(minStr) : 0;

  target.addEventListener("input", (event) => {
    const page = document.querySelector(".page") as HTMLElement;

    const value = parseInt(target.value);

    if (page && value >= min) {
      editor.resize_page(page, value);
    }
  });

  target.addEventListener(
    "mousewheel",
    (event) => {
      const value = parseFloat(target.value) - (event as WheelEvent).deltaY;

      const page = document.querySelector(".page") as HTMLElement;
      if (page && value >= min) {
        target.value = value.toString();

        editor.resize_page(page, value);
      }

      event.preventDefault();
    },
    { passive: false }
  );
}

export function connect(editor: Editor) {
  connect_width_input(editor);

  document
    .getElementById("page-fill-input")
    .addEventListener("input", (event) => {
      const target = event.target as HTMLInputElement;

      const page = document.querySelector(".page") as HTMLElement;
      page.style.backgroundColor = target.value;

      let text = target.value.split("#")[1];
      document.getElementById("page-fill-span").innerText = text.toUpperCase();
    });
}
