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
