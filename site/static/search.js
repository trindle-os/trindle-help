/* Search for Trindle help: plain JavaScript over search-index.js, which the
   build writes. Nothing is sent anywhere; the index is on the page's own
   host. Without this script the search page shows a list of every page. */
(function () {
  "use strict";
  var data = window.SEARCH_INDEX || [];
  var input = document.getElementById("sq");
  var list = document.getElementById("results");
  var status = document.getElementById("status");
  var all = document.getElementById("all");
  var form = document.getElementById("sform");
  if (!input || !list) return;

  function squash(s) { return s.toLowerCase().replace(/[-'’]/g, ""); }
  function words(q) {
    return squash(q).split(/[^a-z0-9À-ɏ]+/).filter(function (w) { return w.length > 0; });
  }
  // A word matches where one of the words in the text starts with it, so
  // "lock" finds "locking" but not "block".
  function has(hay, w) {
    var i = hay.indexOf(w);
    while (i !== -1) {
      if (i === 0 || !/[a-z0-9\u00c0-\u024f]/.test(hay.charAt(i - 1))) return true;
      i = hay.indexOf(w, i + 1);
    }
    return false;
  }
  function at(hay, w, from) {
    var i = hay.indexOf(w, from);
    while (i !== -1) {
      if (i === 0 || !/[a-z0-9\u00c0-\u024f]/.test(hay.charAt(i - 1))) return i;
      i = hay.indexOf(w, i + 1);
    }
    return -1;
  }

  function run(q) {
    var ws = words(q), found = [];
    if (!ws.length) return found;
    data.forEach(function (d) {
      var title = squash(d.t);
      d.c.forEach(function (s, n) {
        var head = squash(s.h), text = squash(s.x), ok = true, score = 0;
        ws.forEach(function (w) {
          var inT = has(title, w), inH = has(head, w), inX = has(text, w);
          if (!(inT || inH || inX)) ok = false;
          if (inT && n === 0) score += 12;
          if (inT) score += 3;
          if (inH) score += 8;
          if (inX) score += 2;
        });
        if (ok) found.push({ d: d, s: s, score: score - n * 0.01 });
      });
    });
    found.sort(function (a, b) { return b.score - a.score; });
    var per = {}, out = [];
    found.forEach(function (f) {
      per[f.d.p] = (per[f.d.p] || 0) + 1;
      if (per[f.d.p] <= 2 && out.length < 15) out.push(f);
    });
    return out;
  }

  function snippet(text, ws) {
    var low = text.toLowerCase().replace(/[-'’]/g, ""), at = -1;
    ws.forEach(function (w) { var i = low.indexOf(w); if (i !== -1 && (at === -1 || i < at)) at = i; });
    var start = Math.max(0, at - 60), end = Math.min(text.length, start + 200);
    return (start > 0 ? "…" : "") + text.slice(start, end) + (end < text.length ? "…" : "");
  }

  function mark(el, text, ws) {
    var low = text.toLowerCase(), i = 0;
    while (i < text.length) {
      var best = -1, len = 0;
      ws.forEach(function (w) {
        var j = at(low, w, i);
        if (j !== -1 && (best === -1 || j < best)) { best = j; len = w.length; }
      });
      if (best === -1) break;
      if (best > i) el.appendChild(document.createTextNode(text.slice(i, best)));
      var m = document.createElement("mark");
      m.textContent = text.slice(best, best + len);
      el.appendChild(m);
      i = best + len;
    }
    if (i < text.length) el.appendChild(document.createTextNode(text.slice(i)));
  }

  function show(q) {
    var ws = words(q), res = run(q);
    list.textContent = "";
    if (!ws.length) {
      status.textContent = "";
      all.className = "";
      return;
    }
    all.className = res.length ? "hide" : "";
    status.textContent = res.length
      ? res.length + (res.length === 1 ? " result" : " results") + " for “" + q.trim() + "”."
      : "Nothing found for “" + q.trim() + "”. Check the spelling, or try fewer or simpler words.";
    res.forEach(function (r) {
      var li = document.createElement("li");
      var a = document.createElement("a");
      a.className = "title";
      a.href = r.d.p + (r.s.i ? "#" + r.s.i : "");
      a.textContent = r.s.h && r.s.h !== r.d.t ? r.d.t + ": " + r.s.h : r.d.t;
      li.appendChild(a);
      var p = document.createElement("p");
      mark(p, snippet(r.s.x, ws), ws);
      li.appendChild(p);
      list.appendChild(li);
    });
  }

  function fromAddress() {
    var m = /[?&]q=([^&]*)/.exec(location.search);
    try { return m ? decodeURIComponent(m[1].replace(/\+/g, " ")) : ""; } catch (e) { return ""; }
  }

  input.value = fromAddress();
  show(input.value);
  input.addEventListener("input", function () {
    show(input.value);
    try { history.replaceState(null, "", input.value ? "?q=" + encodeURIComponent(input.value) : location.pathname); } catch (e) {}
  });
  form.addEventListener("submit", function (e) { e.preventDefault(); show(input.value); });

  // Keyboard: Down from the box goes to the first result; Up and Down move
  // between results; Up from the first goes back to the box. "/" anywhere
  // on the page jumps to the box.
  function links() { return Array.prototype.slice.call(list.querySelectorAll("a")); }
  input.addEventListener("keydown", function (e) {
    if (e.key === "ArrowDown") { var l = links(); if (l.length) { e.preventDefault(); l[0].focus(); } }
  });
  list.addEventListener("keydown", function (e) {
    var l = links(), i = l.indexOf(document.activeElement);
    if (e.key === "ArrowDown" && i < l.length - 1) { e.preventDefault(); l[i + 1].focus(); }
    if (e.key === "ArrowUp") { e.preventDefault(); if (i > 0) l[i - 1].focus(); else input.focus(); }
  });
  document.addEventListener("keydown", function (e) {
    var t = e.target && e.target.tagName;
    if (e.key === "/" && t !== "INPUT" && t !== "TEXTAREA" && !e.ctrlKey && !e.metaKey && !e.altKey) {
      e.preventDefault(); input.focus();
    }
  });
})();
