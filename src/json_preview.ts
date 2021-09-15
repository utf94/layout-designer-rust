import { DataType } from "./index";

type Props = { [key: string]: string };
type Styles = { [key: string]: string };
type Attributes = { [key: string]: string };

interface ComponentData {
  props: Props;
  styles: Styles;
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

export function generate_json(): JsonOutput {
  const page = document.getElementById("page");

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

          style
            .filter((item) => item.length > 0)
            .map((item) => {
              let [key, value] = item.split(":");
              return { key: key.trim(), value: value.trim() };
            })
            .forEach(({ key, value }) => {
              style_json[key] = value;
            });

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
              attributes: {},
              innerText,
            },
          };

          return page;
        });

      const layout: Layout = {
        kind,
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
    framework: "solidjs",
    components: "solidui",
    pages: [
      {
        title: "Home",
        width: page_computed_style.width,
        backgroundColor: page_computed_style.backgroundColor,
        layouts,
      },
    ],
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
