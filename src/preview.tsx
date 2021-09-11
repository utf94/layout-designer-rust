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
    .map((ch) => {
      const new_element = document.createElement(
        "layout-container"
      ) as HTMLElement;

      // @ts-ignore
      new_element.style = ch.style;
      // @ts-ignore
      new_element.classList = ch.classList;

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
