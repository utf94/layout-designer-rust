export function close_preview() {
  const preview_wrapper = document.getElementById("preview-wrapper");
  preview_wrapper.style.opacity = "0";
  preview_wrapper.style.visibility = "hidden";

  const preview = document.getElementById("preview");
  preview.style.transform = "translateY(-100%) scale(0)";
  preview.style.visibility = "hidden";
}

export function open_preview() {
  const page = document.getElementById("page");

  const children = [...page.children];

  const layouts = children
    .filter((ch) => ch.classList.contains("container"))
    .map((ch) => ch as HTMLElement)
    .map((ch) => {
      const new_element = document.createElement(
        "layout-container"
      ) as HTMLElement;

      const style = ch.getAttribute("style");
      new_element.setAttribute("style", style);

      const classes = ch.getAttribute("class");
      new_element.setAttribute("class", classes);

      [...ch.children]
        .filter((ch) => ch.classList.contains("component"))
        // @ts-ignore
        .map((ch) => ch.into_inner())
        .forEach((element) => {
          new_element.appendChild(element);
        });

      return new_element;
    });

  const preview = document.getElementById("preview");
  [...preview.children].forEach((ch) => ch.remove());

  layouts.forEach((l) => preview.appendChild(l));

  const preview_wrapper = document.getElementById("preview-wrapper");
  preview_wrapper.style.opacity = "1";
  preview_wrapper.style.visibility = "visible";

  preview.style.transform = "translateY(0) scale(1)";
  preview.style.visibility = "visible";
}

export function connect() {
  document
    .querySelector("#preview-close-trigger")
    .addEventListener("click", () => {
      close_preview();
    });

  document
    .querySelector("#preview-open-trigger")
    .addEventListener("click", () => {
      open_preview();
    });
}
