import { DataType } from "./index";
import { TailwindConfig } from "./parser/parser";

type Props = { [key: string]: string };
type Styles = { [key: string]: string };
type Attributes = { [key: string]: string };

interface ComponentData {
  props: Props;
  classes: string[];
  attributes: Attributes;
  innerText: string | null;
}

interface Component {
  name: string;
  id: string;
  data: ComponentData;
}

enum LayoutKind {
  Free = "free",
  Flex = "flex",
  Grid = "grid",
}

interface Layout {
  classes: string[];
  kind: LayoutKind;
  components: Component[];
}

interface Page {
  title: string;
  classes: string[];
  layouts: Layout[];
}

interface JsonOutput {
  framework: string;
  components: string;
  pages: Page[];
  tailwindConfig: any;
}

//
//
//

interface ComponentParameter {
  name: string;
  data_type: DataType;
}

interface ComponentDescriptor {
  tag_name: string;
  parameters: ComponentParameter[];
}

interface EditorComponent extends HTMLElement {
  descriptor: ComponentDescriptor;
}

function generate_component_json(
  component: EditorComponent,
  layout_kind: LayoutKind,
  config: TailwindConfig
): Component {
  const desc = component.descriptor;

  const classes: string[] = [];

  if (layout_kind == LayoutKind.Free) {
    classes.push("absolute");

    const computedStyle = window.getComputedStyle(component);
    const top = parseInt(computedStyle.top);
    const left = parseInt(computedStyle.left);

    classes.push("top-" + config.getSpacingName(top));
    classes.push("left-" + config.getSpacingName(left));
  } else if (layout_kind == LayoutKind.Grid) {
    classes.push("w-full");
    classes.push("h-full");

    const computedStyle = window.getComputedStyle(component);

    const colStart = computedStyle.gridColumnStart;
    let colSpan = "1";

    {
      const split = computedStyle.gridColumnEnd.split(" ");
      if (split.length === 2) {
        if (split[0] === "span") {
          colSpan = split[1];
        }
      }
    }

    const rowStart = computedStyle.gridRowStart;
    let rowSpan = "1";

    {
      const split = computedStyle.gridRowEnd.split(" ");
      if (split.length === 2) {
        if (split[0] === "span") {
          rowSpan = split[1];
        }
      }
    }

    classes.push(
      ...[
        `col-start-${colStart}`,
        `col-span-${colSpan}`,
        `row-start-${rowStart}`,
        `row-span-${rowSpan}`,
      ]
    );
  }

  const props: Props = {};
  let innerText: string | null = null;

  desc.parameters.forEach((param) => {
    const value = component.getAttribute(param.name);

    if (param.name === "innertext") {
      if (value?.length > 0) {
        innerText = value;
      } else {
        innerText = null;
      }
    } else {
      if (value?.length > 0) {
        props[param.name] = value;
      } else {
        props[param.name] = null;
      }
    }
  });

  return {
    name: desc.tag_name,
    id: component.id,
    data: {
      props,
      classes,
      attributes: {},
      innerText,
    },
  };
}

function generate_layout_json(
  layout: HTMLElement,
  config: TailwindConfig
): Layout {
  let kind: LayoutKind;

  if (layout.classList.contains("free")) {
    kind = LayoutKind.Free;
  } else if (layout.classList.contains("flex")) {
    kind = LayoutKind.Flex;
  } else if (layout.classList.contains("grid")) {
    kind = LayoutKind.Grid;
  } else {
    kind = LayoutKind.Free;
  }

  const components = [...layout.children]
    .filter((ch) => ch.classList.contains("component"))
    .map((ch) => ch as EditorComponent)
    .map((c) => generate_component_json(c, kind, config));

  const height = parseInt(window.getComputedStyle(layout).height);
  const classes: string[] = ["h-" + config.getSpacingName(height)];

  layout.classList.forEach((value) => {
    if (value !== "free" && value !== "container") {
      classes.push(value);
    }
  });

  if (kind == LayoutKind.Grid) {
    const col = layout.style.getPropertyValue("grid-template-columns");
    const colVal = parseInt(col.split(",")[1].split("px")[0]);

    const colName = config.getGridColName(colVal);
    classes.push("grid-cols-" + colName);

    const row = layout.style.getPropertyValue("grid-template-rows");
    const rowVal = parseInt(row.split(",")[1].split("px")[0]);

    const rowName = config.getGridRowName(rowVal);
    classes.push("grid-rows-" + rowName);
  }

  return {
    kind,

    classes,

    components,
  };
}

function generate_page_json(page: HTMLElement, config: TailwindConfig): Page {
  const children = [...page.children];

  const layouts = children
    .filter((ch) => ch.classList.contains("container"))
    .map((ch) => ch as HTMLElement)
    .map((ch) => generate_layout_json(ch, config));

  const page_computed_style = window.getComputedStyle(page);
  const width = parseInt(page_computed_style.width);

  function rgbToHex(r: number, g: number, b: number) {
    function componentToHex(c: number) {
      var hex = c.toString(16);
      return hex.length == 1 ? "0" + hex : hex;
    }
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
  }

  // TODO: Replace this temporary color getter
  const bg = page_computed_style.backgroundColor.substr(4).split(",");
  const r = parseInt(bg[0]);
  const g = parseInt(bg[1]);
  const b = parseInt(bg[2]);

  return {
    title: "Home",
    classes: [
      "w-" + config.getSpacingName(width),
      "bg-" + config.getColorName(rgbToHex(r, g, b)),
    ],
    layouts,
  };
}

export function generate_json(): JsonOutput {
  let config = new TailwindConfig();

  const pages = [...document.querySelectorAll(".page")].map((page_elm) =>
    generate_page_json(page_elm as HTMLElement, config)
  );

  return {
    framework: "solidjs",
    components: "solidui",
    pages,
    tailwindConfig: config.getTailwindConfig(),
  };
}

function download(text: string) {
  var a = document.createElement("a");
  var file = new Blob([text], { type: "text/json" });
  a.href = URL.createObjectURL(file);
  a.download = "test.json";
  a.click();
}

function download_purs(text: string) {
  var a = document.createElement("a");
  var file = new Blob([text], { type: "text/plain" });
  a.href = URL.createObjectURL(file);
  a.download = "sample.purs";
  a.click();
}

export function connect() {
  document.querySelector("#json-preview-btn").addEventListener("click", () => {
    const json = generate_json();

    download(JSON.stringify(json, null, 4));
  });
  document.querySelector("#ps-preview-btn").addEventListener("click", async () => {
    const debug: {[key: string]: any} = require('../debug.json');
    download_purs(debug.data);
  });
}
