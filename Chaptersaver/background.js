// Función para guardar bookmark
function saveBookMark(url) {
  if (!url) {
    return;
  }
  browser.bookmarks.create({ title: "One Piece", url }).catch(onError);
}

// Función para actualizar bookmark (solo cambia la URL, el título es fijo)
function updateChapterBookMark(url) {
  browser.bookmarks.search({ title: "One Piece" })
    .then(results => {
      if (results.length > 0) {
        browser.bookmarks.update(results[0].id, { url });
      } else {
        saveBookMark(url);
      }
    })
    .catch(onError);
}

// Función que se ejecuta al cambiar de pestaña o navegar
function handleTabUpdate(tabId, changeInfo, tab) {
  if (changeInfo.status === "complete" && tab.active) {
    if (tab.url.includes("https://ww12.readonepiece.com/chapter")) {
      updateChapterBookMark(tab.url);
    }
  }
}

// Función para manejar errores
function onError(error) {
  console.error(`Error: ${error}`);
}

// Listener para cambios en pestañas
browser.tabs.onUpdated.addListener(handleTabUpdate);

