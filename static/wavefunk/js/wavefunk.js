document.addEventListener('click', event => {
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

document.addEventListener('DOMContentLoaded', wfFormatTimestamps);
document.addEventListener('htmx:afterSettle', wfFormatTimestamps);
