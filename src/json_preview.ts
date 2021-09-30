import { DataType } from "./index";

type Props = { [key: string]: string };
type Styles = { [key: string]: string };
type Attributes = { [key: string]: string };

interface ComponentData {
  props: Props;
  styles: Styles;
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
  styles: Styles;
  kind: LayoutKind;
  height: string;
  components: Component[];
}

interface Page {
  title: string;
  width: string;
  backgroundColor: string;
  layouts: Layout[];
}

interface JsonOutput {
  framework: string;
  components: string;
  pages: Page[];
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
  layout_kind: LayoutKind
): Component {
  const desc = component.descriptor;

  const style = component.getAttribute("style").split(";");

  const style_json: Styles = {};
  const classes: string[] = [];

  style
    .filter((item) => item.length > 0)
    .map((item) => {
      let [key, value] = item.split(":");
      return { key: key.trim(), value: value.trim() };
    })
    .forEach(({ key, value }) => {
      style_json[key] = value;
    });

  if (layout_kind == LayoutKind.Free) {
    classes.push("absolute");

    // if (style_json["top"]) {
    //   classes.push(`top-${style_json["top"]}`);
    // } else {
    //   classes.push("top-0");
    // }

    // if (style_json["left"]) {
    //   classes.push(`left-${style_json["left"]}`);
    // } else {
    //   classes.push("left-0");
    // }
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

    if (style_json["grid-area"]) {
      delete style_json["grid-area"];
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
      styles: style_json,
      classes,
      attributes: {},
      innerText,
    },
  };
}

function generate_layout_json(layout: HTMLElement): Layout {
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

  const height = window.getComputedStyle(layout).height;

  const components = [...layout.children]
    .filter((ch) => ch.classList.contains("component"))
    .map((ch) => ch as EditorComponent)
    .map((c) => generate_component_json(c, kind));

  const classes: string[] = [];
  layout.classList.forEach((value) => {
    if (value !== "free" && value !== "container") {
      classes.push(value);
    }
  });

  const styles: Styles = {};
  if (kind == LayoutKind.Grid) {
    styles["grid-template-columns"] = layout.style.getPropertyValue(
      "grid-template-columns"
    );
    styles["grid-template-rows"] =
      layout.style.getPropertyValue("grid-template-rows");
  }

  return {
    kind,

    classes,
    styles,

    height,
    components,
  };
}

function generate_page_json(page: HTMLElement): Page {
  const children = [...page.children];

  const layouts = children
    .filter((ch) => ch.classList.contains("container"))
    .map((ch) => ch as HTMLElement)
    .map(generate_layout_json);

  const page_computed_style = window.getComputedStyle(page);

  return {
    title: "Home",
    width: page_computed_style.width,
    backgroundColor: page_computed_style.backgroundColor,
    layouts,
  };
}

export function generate_json(): JsonOutput {
  const pages = [...document.querySelectorAll(".page")].map((page_elm) =>
    generate_page_json(page_elm as HTMLElement)
  );

  return {
    framework: "solidjs",
    components: "solidui",
    pages,
  };
}

function download(text: string) {
  var a = document.createElement("a");
  var file = new Blob([text], { type: "text/json" });
  a.href = URL.createObjectURL(file);
  a.download = "test.json";
  a.click();
}

export function connect() {
  document.querySelector("#json-preview-btn").addEventListener("click", () => {
    const json = generate_json();

    download(JSON.stringify(json, null, 4));
  });
}
