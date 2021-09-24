import { ContextMenu } from "./context_menu";

class Grid {
  private svg: SVGSVGElement;
  private placeholder: HTMLElement;

  constructor() {
    this.svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");

    this.svg.classList.add("grid-svg");
    this.svg.setAttribute("width", "100%");
    this.svg.setAttribute("height", "100%");
    this.svg.setAttribute("style", "position: absolute; pointer-events: none");

    this.svg.innerHTML = `
              <rect
                width="100%"
                height="100%"
                fill="url(#grid-pattern)"
              ></rect>`;

    this.placeholder = document.createElement("div");
    this.placeholder.className = "grid-placeholder";
  }

  mount(root) {
    root.prepend(this.placeholder);
    root.prepend(this.svg);
  }

  remove() {
    this.svg.remove();
    this.placeholder.remove();
  }
}

class LayoutContainer extends HTMLElement {
  private grid: Grid;

  constructor() {
    super();
    this.grid = new Grid();
  }

  connectedCallback() {
    this.classList.add("container");

    {
      const div = document.createElement("div");
      div.classList.add("container__close-icon");

      const img = document.createElement("img");
      img.src = "/img/icons/close.svg";

      div.appendChild(img);

      this.prepend(div);
    }

    if (this.classList.contains("grid")) {
      this.grid.mount(this);
    } else {
      this.grid.remove();
    }
  }

  open_contextmenu(event) {
    const context_menu = document.querySelector("context-menu") as ContextMenu;

    const root = document.createElement("div");

    const select = document.createElement("div");
    select.className = "layout-select";

    const free_btn = document.createElement("button");
    free_btn.innerText = "Free";

    free_btn.addEventListener("click", () => {
      this.classList.add("free");
      this.classList.remove("flex");
      this.classList.remove("grid");
      this.grid.remove();
      context_menu.close();
    });

    const flex_btn = document.createElement("button");
    flex_btn.innerText = "Flex";

    flex_btn.addEventListener("click", () => {
      this.classList.remove("free");
      this.classList.add("flex");
      this.classList.remove("grid");
      this.grid.remove();
      context_menu.close();
    });

    const grid_btn = document.createElement("button");
    grid_btn.innerText = "Grid";

    grid_btn.addEventListener("click", () => {
      this.classList.remove("free");
      this.classList.remove("flex");
      this.classList.add("grid");
      this.grid.mount(this);
      context_menu.close();
    });

    select.appendChild(free_btn);
    select.appendChild(flex_btn);
    select.appendChild(grid_btn);

    root.appendChild(select);

    if (this.classList.contains("free")) {
      free_btn.classList.add("active");
    } else if (this.classList.contains("flex")) {
      flex_btn.classList.add("active");

      const div = document.createElement("div");
      div.style.display = "flex";
      div.style.marginTop = "10px";

      {
        const label = document.createElement("span");
        label.innerText = "Justify: ";
        label.style.marginRight = "5px";
        label.style.fontSize = "13px";
        div.appendChild(label);
      }

      {
        const select = document.createElement("select");

        select.innerHTML = `
        <optgroup>
          <option value="flex-start">Start</option>
          <option value="center">Center</option>
          <option value="flex-end">End</option>
        </optgroup>

        <optgroup>
          <option value="space-between">Space Between</option>
          <option value="space-around">Space Around</option>
          <option value="space-evenly" selected>Space Evenly</option>
        </optgroup>
      `;

        const justify = this.style.justifyContent;
        select.value = justify.length !== 0 ? justify : "space-evenly";

        select.addEventListener("change", (event) => {
          // @ts-ignore
          this.style.justifyContent = event.target.value;
          context_menu.close();
        });

        div.appendChild(select);
      }

      root.appendChild(div);
    } else if (this.classList.contains("grid")) {
      grid_btn.classList.add("active");
    }

    {
      const sizer = document.createElement("div");
      sizer.style.display = "flex";
      sizer.style.marginTop = "10px";

      const label = document.createElement("span");
      label.innerText = "Size: ";
      label.style.marginRight = "5px";

      sizer.appendChild(label);

      const input_y = document.createElement("input");
      input_y.type = "range";
      input_y.min = "75";
      input_y.max = "700";

      const height = this.style.height.split("px")[0];
      input_y.value = height ? height : "75";

      input_y.addEventListener("input", (event) => {
        // @ts-ignore
        this.style.height = event.target.value + "px";
      });

      sizer.appendChild(input_y);

      root.appendChild(sizer);
    }

    context_menu.open(event, root);
  }
}

customElements.define("layout-container", LayoutContainer);
