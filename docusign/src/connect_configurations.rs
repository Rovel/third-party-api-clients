use anyhow::Result;

use crate::Client;

pub struct ConnectConfigurations {
    pub client: Client,
}

impl ConnectConfigurations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ConnectConfigurations { client }
    }

    /**
     * Get Connect Configuration Information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/connect` endpoint.
     *
     * Retrieves all the DocuSign Custom Connect definitions for the specified account.
     *
     * **Note**: Connect must be enabled for your account to use this function. This does not retrieve information for Connect configurations for Box, eOriginal, or Salesforce.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_get_config(
        &self,
        account_id: &str,
    ) -> Result<crate::types::ConnectConfigResults> {
        let url = format!(
            "/v2.1/accounts/{}/connect",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Updates a specified Connect configuration.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/connect` endpoint.
     *
     * Updates the specified DocuSign Connect configuration in your account.
     *
     * **Note**: Connect must be enabled for your account to use this function. This cannot be used to update Connect configurations for Box, eOriginal, or Salesforce.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_put_configuration(
        &self,
        account_id: &str,
        body: &crate::types::ConnectCustomConfiguration,
    ) -> Result<crate::types::ConnectCustomConfiguration> {
        let url = format!(
            "/v2.1/accounts/{}/connect",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Creates a connect configuration for the specified account.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/connect` endpoint.
     *
     * Creates a DocuSign Custom Connect definition for your account. DocuSign Connect enables the sending of real-time data updates to external applications. These updates are generated by user transactions as the envelope progresses through actions to completion. The Connect Service provides updated information about the status of these transactions and returns updates that include the actual content of document form fields. Be aware that, these updates might or might not include the document itself. For more information about Connect, see the [DocuSign Connect](https://developers.docusign.com/esign-rest-api/guides/connect).
     *
     * **Note**: Connect must be enabled for your account to use this function. This cannot be used to set up Connect configurations for Salesforce or eOriginal.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_post_configuration(
        &self,
        account_id: &str,
        body: &crate::types::ConnectCustomConfiguration,
    ) -> Result<crate::types::ConnectCustomConfiguration> {
        let url = format!(
            "/v2.1/accounts/{}/connect",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Gets the details about a Connect configuration.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/connect/{connectId}` endpoint.
     *
     * Retrieves the information for the specified DocuSign Connect configuration.
     *
     * **Note**: Connect must be enabled for your account to use this function.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `connect_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_get_config_connect_configurations(
        &self,
        account_id: &str,
        connect_id: &str,
    ) -> Result<crate::types::ConnectConfigResults> {
        let url = format!(
            "/v2.1/accounts/{}/connect/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&connect_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Deletes the specified connect configuration.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/connect/{connectId}` endpoint.
     *
     * Deletes the specified DocuSign Connect configuration.
     *
     * **Note**: Connect must be enabled for your account to use this function.
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `connect_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_delete_config(&self, account_id: &str, connect_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/connect/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&connect_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Returns users from the configured Connect service.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/connect/{connectId}/users` endpoint.
     *
     * Returns users from the configured Connect service.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `connect_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `email_substring: &str` -- Filters returned user records by full email address or a substring of email address.
     * * `list_included_users: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The position within the total result set from which to start returning values. The value **thumbnail** may be used to return the page image.
     * * `status: &str` -- Filters the results by user status.
     *   You can specify a comma-separated
     *   list of the following statuses:
     *   
     *   * ActivationRequired
     *   * ActivationSent
     *   * Active
     *   * Closed
     *   * Disabled
     *   .
     * * `user_name_substring: &str` -- Filters results based on a full or partial user name.
     *   
     *   **Note**: When you enter a partial user name, you do not use a wildcard character.
     */
    pub async fn connect_get_user(
        &self,
        account_id: &str,
        connect_id: &str,
        count: &str,
        email_substring: &str,
        list_included_users: &str,
        start_position: &str,
        status: &str,
        user_name_substring: &str,
    ) -> Result<crate::types::IntegratedUserInfoList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !email_substring.is_empty() {
            query_args.push(("email_substring".to_string(), email_substring.to_string()));
        }
        if !list_included_users.is_empty() {
            query_args.push((
                "list_included_users".to_string(),
                list_included_users.to_string(),
            ));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !user_name_substring.is_empty() {
            query_args.push((
                "user_name_substring".to_string(),
                user_name_substring.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v2.1/accounts/{}/connect/{}/users?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&connect_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
