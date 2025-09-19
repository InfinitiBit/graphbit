document.addEventListener("DOMContentLoaded", async () => {
  const repoOwner = "InfinitiBit";
  const repoName = "graphbit";

  try {
    const res = await fetch(`https://api.github.com/repos/${repoOwner}/${repoName}`);
    if (!res.ok) throw new Error("GitHub API request failed");

    const data = await res.json();
    const stars = data.stargazers_count.toLocaleString();
    const forks = data.forks_count.toLocaleString();

    // Fetch latest release
    let version = "N/A";
    try {
      // Fallback: get latest tag
      const tagsRes = await fetch(`https://api.github.com/repos/${repoOwner}/${repoName}/tags`);
      if (tagsRes.ok) {
        const tagsData = await tagsRes.json();
        if (tagsData.length > 0) {
          version = tagsData[0].name;
        }
      }
    } catch {}

    // Create container <ul>
    const container = document.createElement("ul");
    container.className = "md-source__facts";

    // Version
    const versionLi = document.createElement("li");
    versionLi.className = "md-source__fact md-source__fact--version";
    versionLi.innerHTML = `${version}`;
    container.appendChild(versionLi);

    // Stars
    const starsLi = document.createElement("li");
    starsLi.className = "md-source__fact md-source__fact--stars";
    starsLi.innerHTML = `${stars}`;
    container.appendChild(starsLi);

    // Forks
    const forksLi = document.createElement("li");
    forksLi.className = "md-source__fact md-source__fact--forks";
    forksLi.innerHTML = `${forks}`;
    container.appendChild(forksLi);

    // Append to header container
    const headerContainer = document.querySelector("#gh-stats-container");
    if (headerContainer) {
      headerContainer.appendChild(container);
    }
  } catch (err) {
    console.error("Failed to fetch GitHub repo stats:", err);
  }
});
