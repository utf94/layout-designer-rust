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

interface Padding {
  top: number;
  right: number;
  bottom: number;
  left: number;
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
  padding: Padding;
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

function generate_page_json(page: HTMLElement): Page {
  const children = [...page.children];

  const layouts = children
    .filter((ch) => ch.classList.contains("container"))
    .map((ch) => ch as HTMLElement)
    .map((ch) => {
      let kind: LayoutKind;

      if (ch.classList.contains("free")) {
        kind = LayoutKind.Free;
      } else if (ch.classList.contains("flex")) {
        kind = LayoutKind.Flex;
      } else if (ch.classList.contains("grid")) {
        kind = LayoutKind.Grid;
      } else {
        kind = LayoutKind.Free;
      }

      const height = window.getComputedStyle(ch).height;

      const components = [...ch.children]
        .filter((ch) => ch.classList.contains("component"))
        .map((ch) => ch as EditorComponent)
        .map((ch) => {
          const desc = ch.descriptor;

          const style = ch.getAttribute("style").split(";");

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

          if (kind == LayoutKind.Free) {
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
          } else if (kind == LayoutKind.Grid) {
            classes.push("w-full");
            classes.push("h-full");
          }

          const props: Props = {};
          let innerText: string | null = null;

          desc.parameters.forEach((param) => {
            const value = ch.getAttribute(param.name);

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

          const page: Component = {
            name: desc.tag_name,
            id: ch.id,
            data: {
              props,
              styles: style_json,
              classes,
              attributes: {},
              innerText,
            },
          };

          return page;
        });

      const classes: string[] = [];
      ch.classList.forEach((value) => {
        if (value !== "free" && value !== "container") {
          classes.push(value);
        }
      });

      const styles: Styles = {};
      if (kind == LayoutKind.Grid) {
        styles["grid-template-columns"] = ch.style.getPropertyValue(
          "grid-template-columns"
        );
        styles["grid-template-rows"] =
          ch.style.getPropertyValue("grid-template-rows");
      }

      const layout: Layout = {
        kind,

        classes,
        styles,

        height,
        components,
        padding: {
          top: 0,
          right: 0,
          bottom: 0,
          left: 0,
        },
      };

      return layout;
    });

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
