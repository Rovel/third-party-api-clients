use anyhow::Result;

use crate::Client;

pub struct Payments {
    client: Client,
}

impl Payments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Payments { client }
    }

    /**
     * Gets payment information for one or more payments.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_payments` endpoint.
     *
     * Retrieves a list containing information about one or more payments. If the from date or to date queries are not used, the response returns payment information for the last 365 days.
     *
     * Privileges required: account administrator
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `from_date: &str` -- Specifies the date/time of the earliest payment in the account to retrieve.
     * * `to_date: &str` -- Specifies the date/time of the latest payment in the account to retrieve.
     */
    pub async fn billing_get_payment_list(
        &self,
        account_id: &str,
        from_date: &str,
        to_date: &str,
    ) -> Result<crate::types::BillingPaymentsResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !from_date.is_empty() {
            query_args.push(format!("from_date={}", from_date));
        }
        if !to_date.is_empty() {
            query_args.push(format!("to_date={}", to_date));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/billing_payments?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * Posts a payment to a past due invoice.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/billing_payments` endpoint.
     *
     * Posts a payment to a past due invoice.
     *
     * This method can only be used if the `paymentAllowed` value for a past due invoice is true. This can be determined calling [Billing::listInvoicesPastDue](https://developers.docusign.com/docs/esign-rest-api/reference/Billing/Invoices/listPastDue).
     *
     * The response returns information for a single payment, if a payment ID was used in the endpoint, or a list of payments. If the from date or to date queries or payment ID are not used, the response returns payment information for the last 365 days. If the request was for a single payment ID, the `nextUri` and `previousUri` properties are not returned.
     *
     * Privileges required: account administrator
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn billing_post_payment(
        &self,
        account_id: &str,
        body: &crate::types::BillingPaymentRequest,
    ) -> Result<crate::types::BillingPaymentResponse> {
        let url = format!(
            "/v2.1/accounts/{}/billing_payments",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Gets billing payment information for a specific payment.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_payments/{paymentId}` endpoint.
     *
     * Retrieves the information for a specified payment.
     *
     * Privileges required: account administrator
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `payment_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn billing_get_payment(
        &self,
        account_id: &str,
        payment_id: &str,
    ) -> Result<crate::types::BillingPaymentItem> {
        let url = format!(
            "/v2.1/accounts/{}/billing_payments/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&payment_id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
