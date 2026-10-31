class InstCard extends HTMLElement {
     connectedCallback() {
          const name = this.getAttribute('name');
          const src = this.getAttribute('src') || '/assets/images/icon.png';
          const alt = this.getAttribute('alt') || name;
          const button = document.createElement('button');
          button.type = 'button';
          button.dataset.name = name;
          button.className = 'instance-card flex w-full cursor-pointer rounded-md bg-instance-button/95 px-0 ml-1.25 transition duration-300 ease-in-out hover:shadow-[0_0_10px] shadow-black/40';
          const iconWrap = document.createElement('span');
          iconWrap.className = 'shrink-0 pl-2.25 py-[10px]';
          const icon = document.createElement('img');
          icon.width = 30;
          icon.alt = alt;
          icon.loading = "lazy";
          icon.decoding = "async";
          try {
               const parsed = new URL(src, document.baseURI);
               icon.src = parsed.origin === window.location.origin ? parsed.href : '/assets/images/icon.png';
          } catch {
               icon.src = '/assets/images/icon.png';
          }
          iconWrap.append(icon);

          const titleWrap = document.createElement('span');
          titleWrap.className = 'flex min-w-0 items-center';
          const title = document.createElement('span');
          title.className = 'select-none overflow-hidden text-ellipsis whitespace-nowrap pl-2.25 text-[20px] text-white';
          title.textContent = name;
          titleWrap.append(title);

          const menuWrap = document.createElement('span');
          menuWrap.className = 'ml-auto flex shrink-0 pr-1.25 hover:bg-white/10';
          const menu = document.createElement('img');
          menu.src = '/assets/images/three-dots.svg';
          menu.alt = 'Dropdown';
          menu.width = 16;
          menu.loading = "lazy";
          menu.decoding = "async";
          menuWrap.append(menu);

          button.append(iconWrap, titleWrap, menuWrap);
          this.replaceChildren(button);
     }
}

if (!customElements.get('inst-card')) {
     customElements.define('inst-card', InstCard);
}