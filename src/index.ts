import { default as init, Editor } from "../rust/dist/web.js";
import * as Preview from "./preview";
import * as JsonPreview from "./json_preview";

import * as ContextMenu from "./context_menu";
import "./container";

export enum DataType {
  Color = "color",
  Text = "text",
  Number = "number",
  Boolean = "boolean",
  LayoutStyle = "layout_style",
}

export async function run() {
  ContextMenu.connect();

  Preview.connect();
  JsonPreview.connect();

  await init("./dist/web_bg.wasm");
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
      innertext: DataType.Text,
      variant: DataType.Text,
      type: DataType.Text,
      size: DataType.Text,
      shape: DataType.Text,
      disabled: DataType.Boolean,
    },
  });
  editor.register_component({
    tag_name: "launch-checkbox",
    parameters: {
      disabled: DataType.Boolean,
      indeterminate: DataType.Boolean,
    },
  });

  editor.register_component({
    tag_name: "launch-text",
    parameters: {
      innertext: DataType.Text,
      type: DataType.Text,
      weight: DataType.Text,
      underline: DataType.Boolean,
      italic: DataType.Boolean,
    },
  });

  editor.register_component({
    tag_name: "launch-table",
    parameters: {},
  });
}
