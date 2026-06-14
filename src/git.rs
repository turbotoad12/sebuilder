use git2::{Error, Repository};

fn clone_tag(repo_url: &str, tag: &str, dest: &str) -> Result<(), Error> {
    // 1. Clone the repo (shallow clone optional)
    let repo = Repository::clone(repo_url, dest)?;

    // 2. Resolve the tag reference
    let tag_ref = format!("refs/tags/{}", tag);
    let reference = repo.find_reference(&tag_ref)?;

    // 3. Peel the tag to the commit it points to
    let object = reference.peel_to_commit()?;

    // 4. Check out the commit into the working directory
    repo.checkout_tree(object.as_object(), None)?;
    repo.set_head_detached(object.id())?;

    Ok(())
}
