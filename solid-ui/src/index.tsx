import * as SolidUI from "@launch/solid";

import { withSolid } from "solid-element";
import {
  noShadowDOM,
  register,
  PropsDefinitionInput,
  ComponentType,
} from "component-register";

function customElement<T>(
  tag: string,
  props: PropsDefinitionInput<T> | ComponentType<T>,
  ComponentType?: ComponentType<T>
): (ComponentType: ComponentType<T>) => any {
  if (arguments.length === 2) {
    ComponentType = props as ComponentType<T>;
    props = {} as PropsDefinitionInput<T>;
  }

  const r = register<T>(tag, props as PropsDefinitionInput<T>);

  const s = r((a1, a2) => {
    noShadowDOM();
    return (withSolid(ComponentType!) as any)(a1, a2);
  });

  return s;
}

customElement(
  "launch-button",
  {
    variant: "primary",
    type: "filled",
    size: "medium",
    shape: "rounded",
    disabled: "",
  },
  (props, _other) => {
    return <SolidUI.Button {...(props as any)}>Test</SolidUI.Button>;
  }
);

customElement(
  "launch-checkbox",
  {
    disabled: "",
    indeterminate: "",
  },
  (props, _other) => {
    return <SolidUI.Checkbox {...(props as any)}>Test</SolidUI.Checkbox>;
  }
);

customElement(
  "launch-text",
  {
    text: "text",
    type: "normal",
    weight: "normal",
    underline: "",
    italic: "",
  },
  (props, _other) => {
    return (
      <SolidUI.Text {...(props as any)}>
        <slot>{props.text}</slot>
      </SolidUI.Text>
    );
  }
);

// declare type TableProps<Record> = {
//   class?: string;
//   block?: boolean;
//   columns: Column<Record>[];
//   data: Record[];
//   rowSelection?: RowSelection<Record>;
//   loading?: boolean;
// };

customElement("launch-table", {}, (props, _other) => {
  return (
    <SolidUI.Table
      columns={[
        {
          title: "Name",
          key: "name",
          dataIndex: "name",
        },
        {
          title: "Age",
          key: "age",
          dataIndex: "age",
        },
        {
          title: "Origin",
          key: "origin",
          dataIndex: "origin",
        },
      ]}
      data={[
        {
          key: "first",
          name: "XYZ",
          age: 21,
          origin: "Australia",
        },
        {
          key: "second",
          name: "XYZ",
          age: 27,
          origin: "Vancouver",
        },
      ]}
    ></SolidUI.Table>
  );
});
