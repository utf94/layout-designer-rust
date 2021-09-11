import { default as init, Editor } from "../rust/dist/web.js";
import { open_preview, close_preview } from "./preview";
import * as ContextMenu from "./context_menu";
import "./container";

const Type = {
  Color: "color",
  Text: "text",
  Number: "number",
  Boolean: "boolean",
  LayoutStyle: "layout_style",
};

export async function run() {
  ContextMenu.connect();

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

  await init("./rust/dist/web_bg.wasm");
  const editor = new Editor();
  // editor.register_component({
  //   tag_name: "mat-button",
  //   parameters: {
  //     color: Type.Color,
  //     label: Type.Text,
  //   },
  // });
  // editor.register_component({
  //   tag_name: "mat-button-round",
  //   parameters: {
  //     color: Type.Color,
  //   },
  // });
  // editor.register_component({
  //   tag_name: "mat-table",
  //   parameters: {
  //     x: Type.Number,
  //     y: Type.Number,
  //   },
  // });
  editor.register_component({
    tag_name: "launch-button",
    parameters: {
      variant: Type.Text,
      type: Type.Text,
      size: Type.Text,
      shape: Type.Text,
      disabled: Type.Boolean,
    },
  });
  editor.register_component({
    tag_name: "launch-checkbox",
    parameters: {
      disabled: Type.Boolean,
      indeterminate: Type.Boolean,
    },
  });

  editor.register_component({
    tag_name: "launch-text",
    parameters: {
      text: Type.Text,
      type: Type.Text,
      weight: Type.Text,
      underline: Type.Boolean,
      italic: Type.Boolean,
    },
  });
}
