document.addEventListener('click', event => {
  const dismiss = event.target.closest('[data-wf-dismiss="overlay"]');
  if (dismiss) {
    document.querySelectorAll('.wf-overlay.is-open, .wf-modal.is-open, .wf-drawer.is-open')
      .forEach(item => item.classList.remove('is-open'));
    return;
  }

  const copy = event.target.closest('[data-wf-copy]');
  if (copy) {
    event.preventDefault();
    wfCopyValue(copy);
    return;
  }

  const trigger = event.target.closest('[data-popover-toggle]');
  if (trigger) {
    const anchor = trigger.closest('.wf-pop-anchor');
    const popover = anchor && anchor.querySelector('.wf-popover');
    if (popover) {
      const wasOpen = popover.classList.contains('is-open');
      document.querySelectorAll('.wf-popover.is-open').forEach(item => item.classList.remove('is-open'));
      if (!wasOpen) popover.classList.add('is-open');
    }
    return;
  }

  if (!event.target.closest('.wf-popover')) {
    document.querySelectorAll('.wf-popover.is-open').forEach(item => item.classList.remove('is-open'));
  }
});

function wfCopyText(text) {
  if (navigator.clipboard && navigator.clipboard.writeText) {
    return navigator.clipboard.writeText(text).catch(() => wfFallbackCopyText(text));
  }

  return wfFallbackCopyText(text);
}

function wfFallbackCopyText(text) {
  const area = document.createElement('textarea');
  area.value = text;
  area.setAttribute('readonly', '');
  area.style.position = 'fixed';
  area.style.opacity = '0';
  document.body.appendChild(area);
  area.select();
  document.execCommand('copy');
  area.remove();
  return Promise.resolve();
}

function wfCopyValue(trigger) {
  const selector = trigger.getAttribute('data-wf-copy');
  const source = selector ? document.querySelector(selector) : null;
  const text = source && 'value' in source ? source.value : source ? source.textContent : '';
  if (!text) return;

  wfCopyText(text.trim()).then(() => {
    trigger.setAttribute('data-wf-copy-state', 'ok');
    trigger.dispatchEvent(new CustomEvent('wfCopy', { bubbles: true, detail: { text } }));
  }).catch(() => {
    trigger.setAttribute('data-wf-copy-state', 'error');
  });
}

function wfPushToast(kind, msg) {
  const host = document.getElementById('toast-host');
  if (!host || !msg) return;

  const toast = document.createElement('div');
  toast.className = 'wf-toast' + (kind ? ' ' + kind : '');

  const dot = document.createElement('span');
  dot.className = 'wf-dot';

  const label = document.createElement('span');
  label.textContent = msg;

  toast.appendChild(dot);
  toast.appendChild(label);
  host.appendChild(toast);

  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transition = 'opacity 200ms';
    setTimeout(() => toast.remove(), 220);
  }, 2600);
}

document.addEventListener('wfToast', event => {
  const { kind = '', msg = '' } = event.detail || {};
  wfPushToast(kind, msg);
}, true);

document.addEventListener('wfEcho', event => {
  const { kind = '', msg = '' } = event.detail || {};
  document.querySelectorAll('[data-wf-echo]').forEach(target => {
    target.textContent = msg;
    target.dataset.kind = kind;
    target.classList.remove('is-visible', 'is-ok', 'is-warn', 'is-err', 'is-info');
    if (msg) target.classList.add('is-visible');
    if (kind) target.classList.add('is-' + kind);
  });
}, true);

function wfFormatTimestamps() {
  document.querySelectorAll('[data-ts]').forEach(element => {
    const iso = element.getAttribute('data-ts');
    if (!iso) return;

    const diff = Date.now() - new Date(iso).getTime();
    const secs = Math.floor(diff / 1000);
    let text;

    if (secs < 60) text = 'just now';
    else if (secs < 3600) text = Math.floor(secs / 60) + 'm ago';
    else if (secs < 86400) text = Math.floor(secs / 3600) + 'h ago';
    else if (secs < 604800) text = Math.floor(secs / 86400) + 'd ago';
    else text = new Date(iso).toLocaleDateString();

    element.textContent = text;
  });
}

function wfUpdateUploadZone(zone, files) {
  const list = Array.from(files || []);
  const title = zone.querySelector('[data-upload-title], .wf-dropzone-title');
  const hint = zone.querySelector('[data-upload-hint], .wf-dropzone-hint');
  const names = list.map(file => file.name).join(', ');

  zone.classList.remove('is-dragover');
  if (names) zone.setAttribute('data-upload-files', names);
  if (title && names) title.textContent = list.length === 1 ? list[0].name : list.length + ' files selected';
  if (hint && names) hint.textContent = names;
}

function wfInitUploadZones(root = document) {
  root.querySelectorAll('[data-upload-zone]').forEach(zone => {
    if (zone.dataset.wfUploadReady) return;
    zone.dataset.wfUploadReady = 'true';

    const inputSelector = zone.getAttribute('data-upload-input');
    const input = inputSelector ? document.querySelector(inputSelector) : zone.querySelector('input[type="file"]');

    zone.addEventListener('dragover', event => {
      event.preventDefault();
      zone.classList.add('is-dragover');
    });
    zone.addEventListener('dragleave', () => zone.classList.remove('is-dragover'));
    zone.addEventListener('drop', event => {
      event.preventDefault();
      if (input && event.dataTransfer && event.dataTransfer.files.length) {
        input.files = event.dataTransfer.files;
        input.dispatchEvent(new Event('change', { bubbles: true }));
      }
      wfUpdateUploadZone(zone, event.dataTransfer && event.dataTransfer.files);
    });
    if (input) {
      input.addEventListener('change', () => wfUpdateUploadZone(zone, input.files));
    }
  });
}

document.addEventListener('submit', event => {
  const form = event.target.closest('form[data-wf-submit-spinner]');
  if (!form) return;

  form.classList.add('is-submitting');
  const selector = form.getAttribute('data-wf-submit-spinner');
  const spinner = selector ? document.querySelector(selector) : form.querySelector('[data-wf-submit-spinner-target]');
  if (spinner) {
    spinner.hidden = false;
    spinner.classList.add('is-visible');
  }
});

function wfInitDirtyGuards(root = document) {
  root.querySelectorAll('form[data-wf-dirty-guard]').forEach(form => {
    if (form.dataset.wfDirtyReady) return;
    form.dataset.wfDirtyReady = 'true';
    form.addEventListener('input', () => { form.dataset.wfDirty = 'true'; });
    form.addEventListener('change', () => { form.dataset.wfDirty = 'true'; });
    form.addEventListener('submit', () => { form.dataset.wfDirty = 'false'; });
  });
}

window.addEventListener('beforeunload', event => {
  const dirty = document.querySelector('form[data-wf-dirty-guard][data-wf-dirty="true"]');
  if (!dirty) return;
  event.preventDefault();
  event.returnValue = '';
});

function wfUpdateActiveNav() {
  document.querySelectorAll('[data-wf-active-nav]').forEach(nav => {
    nav.querySelectorAll('a[href]').forEach(link => {
      const href = new URL(link.getAttribute('href'), window.location.href);
      link.classList.toggle('is-active', href.pathname === window.location.pathname);
    });
  });
}

function wfRefreshPageChrome(root = document) {
  const title = root.querySelector('#page-title');
  if (title && title.textContent.trim()) document.title = title.textContent.trim();

  root.querySelectorAll('[data-wf-echo-message]').forEach(source => {
    document.dispatchEvent(new CustomEvent('wfEcho', {
      bubbles: true,
      detail: {
        kind: source.getAttribute('data-wf-echo-kind') || '',
        msg: source.getAttribute('data-wf-echo-message') || source.textContent.trim()
      }
    }));
  });
  wfUpdateActiveNav();
}

function wfInit(root = document) {
  wfFormatTimestamps();
  wfInitUploadZones(root);
  wfInitDirtyGuards(root);
  wfRefreshPageChrome(root);
}

document.addEventListener('DOMContentLoaded', () => wfInit(document));
document.addEventListener('htmx:afterSwap', event => wfInit(event.target || document));
document.addEventListener('htmx:afterSettle', wfFormatTimestamps);
