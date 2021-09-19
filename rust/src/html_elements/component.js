export class ComponentParameter {
  constructor(name, type) {
    this._name = name;
    this._type = type;
  }

  get name() {
    return this._name;
  }

  get data_type() {
    return this._type;
  }
}

export class ComponentDescriptor {
  constructor(desc) {
    this._tag_name = desc.tag_name;
    this._parameters = Object.entries(desc.parameters).map(
      ([key, val]) => new ComponentParameter(key, val)
    );
  }

  get tag_name() {
    return this._tag_name;
  }

  get parameters() {
    return this._parameters;
  }
}

export class EditorComponent extends HTMLElement {
  constructor(desc) {
    super();

    this.inited = false;

    if (desc != null) {
      this._descriptor = desc;
    } else {
      throw "EditorComponent requires descriptor";
    }

    this._instance = document.createElement(this._descriptor.tag_name);
  }

  connectedCallback() {
    if (!this.classList.contains("component")) {
      this.classList.add("component");
      this.appendChild(this._instance);

      for (var i = 0; i < this._instance.attributes.length; i++) {
        var attrib = this._instance.attributes[i];
        this.setAttribute(attrib.name, attrib.value);
      }
    }

    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        const type = mutation.type;
        const attributeName = mutation.attributeName;

        if (type === "attributes") {
          if (
            attributeName !== "id" &&
            attributeName !== "class" &&
            attributeName !== "style"
          ) {
            const val = this.getAttribute(attributeName);
            this._instance.setAttribute(attributeName, val);
          }
        }
      });
    });
    observer.observe(this, { attributes: true });
  }

  get instance() {
    return this._instance;
  }

  get descriptor() {
    return this._descriptor;
  }

  into_inner() {
    const tag = this._instance.tagName;
    const node = document.createElement(tag);

    [...this.attributes].forEach((a) => node.setAttribute(a.name, a.value));

    return node;
  }
}

export class EditorComponentSource extends HTMLElement {
  constructor(desc) {
    super();

    if (desc != null) {
      this._descriptor = desc;
    } else {
      throw "EditorComponentSource requires descriptor";
    }
  }

  connectedCallback() {
    this._instance = document.createElement(this._descriptor.tag_name);
    this.appendChild(this._instance);
  }

  get instance() {
    return this._instance;
  }

  get descriptor() {
    return this._descriptor;
  }

  instantiate_component() {
    return new EditorComponent(this._descriptor);
  }
}

export function register() {
  customElements.define("editor-component-source", EditorComponentSource);
  customElements.define("editor-component", EditorComponent);
}
