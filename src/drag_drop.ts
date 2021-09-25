import { Editor } from "./index";

export function connect(editor: Editor) {
  let dragged: HTMLElement;

  /* events fired on the draggable target */
  document.addEventListener("drag", (event) => {}, false);

  document.addEventListener(
    "dragstart",
    (event) => {
      dragged = event.target as HTMLElement;
      dragged.style.opacity = "0.5";

      {
        const elements = document.querySelectorAll("#workspace .page");

        elements.forEach((elm) => {
          const div = document.createElement("div");
          div.classList.add("layout-drop-area");

          div.setAttribute("data-id", "0");

          elm.prepend(div);
        });
      }

      {
        const elements = document.querySelectorAll(
          "#workspace layout-container"
        );

        elements.forEach((elm, id) => {
          const div = document.createElement("div");
          div.classList.add("layout-drop-area");

          div.setAttribute("data-id", (id + 1).toString());

          elm.after(div);
        });
      }
    },
    false
  );

  document.addEventListener(
    "dragend",
    (event) => {
      const target = event.target as HTMLElement;
      target.style.opacity = "";

      const elements = document.querySelectorAll(".layout-drop-area");

      elements.forEach((elm) => elm.remove());
    },
    false
  );

  document.addEventListener(
    "dragover",
    (event) => {
      event.preventDefault();
    },
    false
  );

  document.addEventListener(
    "dragenter",
    (event) => {
      const target = event.target as HTMLElement;

      if (target.classList.contains("layout-drop-area")) {
        target.classList.add("dragover");
      }
    },
    false
  );

  document.addEventListener(
    "dragleave",
    (event) => {
      const target = event.target as HTMLElement;

      if (target.classList.contains("layout-drop-area")) {
        target.classList.remove("dragover");
      }
    },
    false
  );

  document.addEventListener(
    "drop",
    (event) => {
      const target = event.target as HTMLElement;
      event.preventDefault();

      if (target.classList.contains("layout-drop-area")) {
        target.classList.remove("dragover");

        const parent = target.parentElement as HTMLElement;

        if (parent.classList.contains("page")) {
          const id = parseInt(target.getAttribute("data-id"));

          if (!isNaN(id)) {
            const data = event.dataTransfer.getData("text/layout-type");
            if (data) {
              editor.add_layout_to_page(parent, id, data);
            }
          }
        }
      }
    },
    false
  );
}
