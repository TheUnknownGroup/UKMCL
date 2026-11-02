import { invoke } from "@tauri-apps/api/core";
import { deletes, launch, open } from "./instance";

const handlers = {
     launch: async (name: string) => {
          await launch(name);
     },
     reveal: async (name: string) => {
          await open(name);
     },
     delete: async (name: string) => {
          await deletes(name);
     }
}

class InstCard extends HTMLElement {
     connectedCallback() {
          const name = this.getAttribute('name');
          const src = this.getAttribute('src') || '/assets/images/icon.png';
          const alt = this.getAttribute('alt') || name;

          const wrapper = document.createElement('div');
          wrapper.className = "ml-1.25 rounded-md overflow-hidden light:bg-evergreen-500/95 bg-evergreen-800/95 px-0 transition duration-300 ease-in-out hover:shadow-[0_0_10px] shadow-black/40"
          const row = document.createElement('div');
          row.className = "flex";
          
          const button = document.createElement('button');
          button.type = 'button';
          button.dataset.name = name;
          button.className = 'instance-card flex min-w-0 flex-1 cursor-pointer';
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
          title.className = 'select-none overflow-hidden text-ellipsis whitespace-nowrap pl-2.25 text-[20px] light:text-black-forest-800 text-white';
          title.textContent = name;
          titleWrap.append(title);

          button.append(iconWrap, titleWrap);

          const menuBtn = document.createElement('button');
          menuBtn.type = 'button';
          menuBtn.className = 'flex cursor-pointer items-center px-1.25 light:text-black-forest-800 text-white light:hover:bg-black/10 hover:bg-white/10';
          menuBtn.setAttribute('aria-label', 'Instance actions');
          menuBtn.setAttribute('aria-expanded', 'false');
          const menuIcon = document.createElement('i');
          menuIcon.className = "bi bi-three-dots-vertical leading-none";
          menuBtn.append(menuIcon);

          row.append(button, menuBtn);

          const panel = document.createElement('div');
          panel.className = "grid grid-rows-[0fr] transition-[grid-template-rows] duration-200";
          panel.inert = true;
          const clip = document.createElement('div');
          clip.className = "overflow-hidden";
          const list = document.createElement('div');
          list.className = "flex flex-col gap-1 p-1";

          const actions = [
               { action: 'launch', label: 'Launch', icon: 'bi-play-fill'},
               { action: 'reveal', label: 'Reveal in files', icon: 'bi-folder2-open'},
               { action: 'delete', label: 'Delete', icon: 'bi-trash3'},
          ];

          const closeMenu = () => {
               menuBtn.setAttribute('aria-expanded', 'false');
               panel.inert = true;
               panel.classList.remove('grid-rows-[1fr]');
               panel.classList.add('grid-rows-[0fr]');
          }

          for (const a of actions) {
               const item = document.createElement('button');
               item.type = 'button';
               item.dataset.action = a.action;
               item.dataset.name = name;
               item.className = 'flex w-full cursor-pointer items-center gap-2 rounded-md px-3 py-1.5 text-left whitespace-nowrap light:text-black-forest-800 text-white light:hover:bg-black/10 hover:bg-white/10';
               const i = document.createElement('i');
               i.className = `bi ${a.icon}`;
               item.append(i, document.createTextNode(a.label));

               item.onclick = async (e) => {
                    e.stopPropagation();
                    closeMenu();
                    await handlers[a.action]?.(name, this);
               };
               
               list.append(item);
          }

          clip.append(list);
          panel.append(clip);

          menuBtn.addEventListener('click', (e) => {
               e.stopPropagation();
               const open = menuBtn.getAttribute('aria-expanded') !== 'true';
               menuBtn.setAttribute('aria-expanded', String(open));
               panel.inert = !open;
               panel.classList.toggle('grid-rows-[1fr]', open);
               panel.classList.toggle('grid-rows-[0fr]', !open);
          });

          wrapper.append(row, panel);
          this.replaceChildren(wrapper);
     }
}

if (!customElements.get('inst-card')) {
     customElements.define('inst-card', InstCard);
}