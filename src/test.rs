// import functionality form lib

#[cfg(test)]
mod test {
    #[test]
    #[should_panic]
    fn invalid_url_get_request() {
        // test against an empty/non-existent url
        panic!(blindfold::http_get("www.notarealsite/foo/bar"));
    }

    //fn serializing_the_response() {
        // test an invalid json with serde
        // test valid json 
        //
        // test the creation of the filemap
        // are all other files correctly ignored?
    //}

    //fn creating_the_file() {
        // test the title maker function with another language (maybe chinese to demonstrate uni
        // code)
        // 
    //}

    //fn interface() {
       // test the similarity function
       // make sure it always gives a suggestion even with random characters
       // test with numbers
       // test with only one letter difference
       // make sure if a suggestion is denied, that nothing for it is included in the gitignore
    //}
}
