// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Welcome;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Welcome = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZillowEmbeddedJson {
    #[serde(rename = "props")]
    pub props: Option<Props>,

    #[serde(rename = "page")]
    pub page: Option<String>,

    #[serde(rename = "query")]
    pub query: Option<Query>,

    #[serde(rename = "buildId")]
    pub build_id: Option<String>,

    #[serde(rename = "assetPrefix")]
    pub asset_prefix: Option<String>,

    #[serde(rename = "runtimeConfig")]
    pub runtime_config: Option<RuntimeConfig>,

    #[serde(rename = "isFallback")]
    pub is_fallback: Option<bool>,

    #[serde(rename = "gssp")]
    pub gssp: Option<bool>,

    #[serde(rename = "customServer")]
    pub custom_server: Option<bool>,

    #[serde(rename = "appGip")]
    pub app_gip: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Props {
    #[serde(rename = "pageProps")]
    pub page_props: Option<PageProps>,

    #[serde(rename = "pageFrameData")]
    pub page_frame_data: Option<PageFrameData>,

    #[serde(rename = "__N_SSP")]
    pub n_ssp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageFrameData {
    #[serde(rename = "guid")]
    pub guid: Option<String>,

    #[serde(rename = "requestId")]
    pub request_id: Option<String>,

    #[serde(rename = "clientProfilerHostid")]
    pub client_profiler_hostid: Option<String>,

    #[serde(rename = "clientProfilerBeaconUrl")]
    pub client_profiler_beacon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageProps {
    #[serde(rename = "about")]
    pub about: Option<About>,

    #[serde(rename = "breadcrumbs")]
    pub breadcrumbs: Option<Vec<Breadcrumb>>,

    #[serde(rename = "currentUser")]
    pub current_user: Option<CurrentUser>,

    #[serde(rename = "currentUrl")]
    pub current_url: Option<String>,

    #[serde(rename = "displayUser")]
    pub display_user: Option<DisplayUser>,

    #[serde(rename = "lastYearPastSalesTotal")]
    pub last_year_past_sales_total: Option<LastYearPastSalesTotal>,

    #[serde(rename = "forSaleListings")]
    pub for_sale_listings: Option<ForListings>,

    #[serde(rename = "forRentListings")]
    pub for_rent_listings: Option<ForListings>,

    #[serde(rename = "isImpersonating")]
    pub is_impersonating: Option<bool>,

    #[serde(rename = "isReviewModerator")]
    pub is_review_moderator: Option<bool>,

    #[serde(rename = "map")]
    pub map: Option<Map>,

    #[serde(rename = "paginationPageSize")]
    pub pagination_page_size: Option<i64>,

    #[serde(rename = "pastSales")]
    pub past_sales: Option<PastSales>,

    #[serde(rename = "preferredLenders")]
    pub preferred_lenders: Option<PreferredLenders>,

    #[serde(rename = "professionalInformation")]
    pub professional_information: Option<Vec<ProfessionalInformation>>,

    #[serde(rename = "profileDisplay")]
    pub profile_display: Option<ProfileDisplay>,

    #[serde(rename = "reviewsData")]
    pub reviews_data: Option<ReviewsData>,

    #[serde(rename = "seoFooters")]
    pub seo_footers: Option<Vec<SeoFooter>>,

    #[serde(rename = "serviceAreas")]
    pub service_areas: Option<Vec<ServiceArea>>,

    #[serde(rename = "zillowWebHostName")]
    pub zillow_web_host_name: Option<String>,

    #[serde(rename = "zillowStaticHost")]
    pub zillow_static_host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct About {
    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "specialties")]
    pub specialties: Option<Vec<String>>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "yearsExperience")]
    pub years_experience: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breadcrumb {
    #[serde(rename = "text")]
    pub text: Option<String>,

    #[serde(rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    #[serde(rename = "isLoggedIn")]
    pub is_logged_in: Option<bool>,

    #[serde(rename = "loginState")]
    pub login_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayUser {
    #[serde(rename = "encodedZuid")]
    pub encoded_zuid: Option<String>,

    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,

    #[serde(rename = "inCanada")]
    pub in_canada: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "flag")]
    pub flag: Option<i64>,

    #[serde(rename = "profileTypeIds")]
    pub profile_type_ids: Option<Vec<i64>>,

    #[serde(rename = "profileTypes")]
    pub profile_types: Option<Vec<String>>,

    #[serde(rename = "sidebarVideoUrl")]
    pub sidebar_video_url: Option<serde_json::Value>,

    #[serde(rename = "businessAddress")]
    pub business_address: Option<BusinessAddress>,

    #[serde(rename = "businessName")]
    pub business_name: Option<String>,

    #[serde(rename = "cpdUserPronouns")]
    pub cpd_user_pronouns: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessAddress {
    #[serde(rename = "city")]
    pub city: Option<String>,

    #[serde(rename = "state")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForListings {
    #[serde(rename = "listings")]
    pub listings: Option<Vec<Listing>>,

    #[serde(rename = "listing_count")]
    pub listing_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    #[serde(rename = "zpid")]
    pub zpid: Option<i64>,

    #[serde(rename = "home_type")]
    pub home_type: Option<String>,

    #[serde(rename = "address")]
    pub address: Option<Address>,

    #[serde(rename = "bedrooms")]
    pub bedrooms: Option<i64>,

    #[serde(rename = "bathrooms")]
    pub bathrooms: Option<i64>,

    #[serde(rename = "openHouses")]
    pub open_houses: Option<String>,

    #[serde(rename = "hasOpenHouse")]
    pub has_open_house: Option<bool>,

    #[serde(rename = "primary_photo_url")]
    pub primary_photo_url: Option<String>,

    #[serde(rename = "price")]
    pub price: Option<i64>,

    #[serde(rename = "price_currency")]
    pub price_currency: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,

    #[serde(rename = "latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "longitude")]
    pub longitude: Option<f64>,

    #[serde(rename = "brokerage_name")]
    pub brokerage_name: Option<String>,

    #[serde(rename = "home_marketing_status")]
    pub home_marketing_status: Option<String>,

    #[serde(rename = "home_marketing_type")]
    pub home_marketing_type: Option<String>,

    #[serde(rename = "has_vr_model")]
    pub has_vr_model: Option<bool>,

    #[serde(rename = "listing_url")]
    pub listing_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "line1")]
    pub line1: Option<String>,

    #[serde(rename = "line2")]
    pub line2: Option<String>,

    #[serde(rename = "city")]
    pub city: Option<String>,

    #[serde(rename = "stateOrProvince")]
    pub state_or_province: Option<String>,

    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastYearPastSalesTotal {
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "mapCentroid")]
    pub map_centroid: Option<MapCentroid>,

    #[serde(rename = "mapApiKey")]
    pub map_api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCentroid {
    #[serde(rename = "latitude")]
    pub latitude: Option<String>,

    #[serde(rename = "longitude")]
    pub longitude: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastSales {
    #[serde(rename = "total")]
    pub total: Option<i64>,

    #[serde(rename = "past_sales")]
    pub past_sales: Option<Vec<PastSale>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastSale {
    #[serde(rename = "represented")]
    pub represented: Option<String>,

    #[serde(rename = "sold_date")]
    pub sold_date: Option<String>,

    #[serde(rename = "price")]
    pub price: Option<String>,

    #[serde(rename = "zpid")]
    pub zpid: Option<i64>,

    #[serde(rename = "image_url")]
    pub image_url: Option<String>,

    #[serde(rename = "image_alt")]
    pub image_alt: Option<String>,

    #[serde(rename = "home_details_url")]
    pub home_details_url: Option<String>,

    #[serde(rename = "street_address")]
    pub street_address: Option<String>,

    #[serde(rename = "city_state_zipcode")]
    pub city_state_zipcode: Option<String>,

    #[serde(rename = "latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "longitude")]
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferredLenders {
    #[serde(rename = "lenders")]
    pub lenders: Option<Vec<Lender>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lender {
    #[serde(rename = "businessPhoneNumber")]
    pub business_phone_number: Option<String>,

    #[serde(rename = "cellPhoneNumber")]
    pub cell_phone_number: Option<String>,

    #[serde(rename = "displayAccountName")]
    pub display_account_name: Option<bool>,

    #[serde(rename = "employerNMLSId")]
    pub employer_nmls_id: Option<i64>,

    #[serde(rename = "firstName")]
    pub first_name: Option<String>,

    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,

    #[serde(rename = "lastName")]
    pub last_name: Option<String>,

    #[serde(rename = "nmlsLicense")]
    pub nmls_license: Option<i64>,

    #[serde(rename = "ratingAverage")]
    pub rating_average: Option<f64>,

    #[serde(rename = "reviewReceivedCount")]
    pub review_received_count: Option<i64>,

    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,

    #[serde(rename = "website")]
    pub website: Option<String>,

    #[serde(rename = "zuid")]
    pub zuid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionalInformation {
    #[serde(rename = "term")]
    pub term: Option<String>,

    #[serde(rename = "lines")]
    pub lines: Option<Vec<String>>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "links")]
    pub links: Option<Vec<Breadcrumb>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDisplay {
    #[serde(rename = "actionMenu")]
    pub action_menu: Option<ActionMenu>,

    #[serde(rename = "actionToolbar")]
    pub action_toolbar: Option<ActionToolbar>,

    #[serde(rename = "contactCard")]
    pub contact_card: Option<ContactCard>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "private")]
    pub private: Option<bool>,

    #[serde(rename = "profileInfo")]
    pub profile_info: Option<ProfileInfo>,

    #[serde(rename = "teamMembers")]
    pub team_members: Option<ActionMenu>,

    #[serde(rename = "teamSummary")]
    pub team_summary: Option<ActionMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMenu {
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "message")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionToolbar {
    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,

    #[serde(rename = "editProfileInformation")]
    pub edit_profile_information: Option<serde_json::Value>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "isMenu")]
    pub is_menu: Option<bool>,

    #[serde(rename = "isToolbar")]
    pub is_toolbar: Option<bool>,

    #[serde(rename = "publicActionBar")]
    pub public_action_bar: Option<PublicActionBar>,

    #[serde(rename = "requestReviews")]
    pub request_reviews: Option<serde_json::Value>,

    #[serde(rename = "share")]
    pub share: Option<serde_json::Value>,

    #[serde(rename = "switchToPrivate")]
    pub switch_to_private: Option<serde_json::Value>,

    #[serde(rename = "switchToPublic")]
    pub switch_to_public: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicActionBar {
    #[serde(rename = "contact")]
    pub contact: Option<Contact>,

    #[serde(rename = "editProfile")]
    pub edit_profile: Option<serde_json::Value>,

    #[serde(rename = "writeReview")]
    pub write_review: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "contactFormId")]
    pub contact_form_id: Option<String>,

    #[serde(rename = "encodedZuid")]
    pub encoded_zuid: Option<String>,

    #[serde(rename = "lightboxId")]
    pub lightbox_id: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "text")]
    pub text: Option<String>,

    #[serde(rename = "wide")]
    pub wide: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactCard {
    #[serde(rename = "actionBar")]
    pub action_bar: Option<ActionBar>,

    #[serde(rename = "agentTypeDisplay")]
    pub agent_type_display: Option<AgentTypeDisplay>,

    #[serde(rename = "allActivity")]
    pub all_activity: Option<AllActivity>,

    #[serde(rename = "contactCardType")]
    pub contact_card_type: Option<String>,

    #[serde(rename = "editProfilePhoto")]
    pub edit_profile_photo: Option<serde_json::Value>,

    #[serde(rename = "firstName")]
    pub first_name: Option<String>,

    #[serde(rename = "flag")]
    pub flag: Option<Flag>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "isTeamLead")]
    pub is_team_lead: Option<bool>,

    #[serde(rename = "isTeamMember")]
    pub is_team_member: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "profilePath")]
    pub profile_path: Option<String>,

    #[serde(rename = "profilePhotoAlt")]
    pub profile_photo_alt: Option<String>,

    #[serde(rename = "profilePhotoHeight")]
    pub profile_photo_height: Option<i64>,

    #[serde(rename = "profilePhotoIsvalid")]
    pub profile_photo_isvalid: Option<bool>,

    #[serde(rename = "profilePhotoSrc")]
    pub profile_photo_src: Option<String>,

    #[serde(rename = "profilePhotoWidth")]
    pub profile_photo_width: Option<i64>,

    #[serde(rename = "reviewsPath")]
    pub reviews_path: Option<String>,

    #[serde(rename = "teamName")]
    pub team_name: Option<String>,

    #[serde(rename = "teamProfilePath")]
    pub team_profile_path: Option<String>,

    #[serde(rename = "teamRank")]
    pub team_rank: Option<TeamRank>,

    #[serde(rename = "userPronouns")]
    pub user_pronouns: Option<UserPronouns>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionBar {
    #[serde(rename = "contactCCol")]
    pub contact_c_col: Option<ContactCColClass>,

    #[serde(rename = "contactLightbox")]
    pub contact_lightbox: Option<ContactCColClass>,

    #[serde(rename = "contactMobile")]
    pub contact_mobile: Option<serde_json::Value>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "switchToPrivate")]
    pub switch_to_private: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactCColClass {
    #[serde(rename = "contactFormId")]
    pub contact_form_id: Option<String>,

    #[serde(rename = "encodedZuid")]
    pub encoded_zuid: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "text")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTypeDisplay {
    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "text")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllActivity {
    #[serde(rename = "hasReviews")]
    pub has_reviews: Option<bool>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "ratingAverage")]
    pub rating_average: Option<f64>,

    #[serde(rename = "reviews")]
    pub reviews: Option<String>,

    #[serde(rename = "reviewsPath")]
    pub reviews_path: Option<String>,

    #[serde(rename = "sales")]
    pub sales: Option<String>,

    #[serde(rename = "seoReviewCount")]
    pub seo_review_count: Option<i64>,

    #[serde(rename = "seoReviewRating")]
    pub seo_review_rating: Option<f64>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "writeAReviewPath")]
    pub write_a_review_path: Option<String>,

    #[serde(rename = "zsgReviewStar")]
    pub zsg_review_star: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flag {
    #[serde(rename = "contentComponentName")]
    pub content_component_name: Option<String>,

    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    #[serde(rename = "encodedZuid")]
    pub encoded_zuid: Option<String>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "loadLegalTextDynamically")]
    pub load_legal_text_dynamically: Option<bool>,

    #[serde(rename = "originatingUrl")]
    pub originating_url: Option<String>,

    #[serde(rename = "reporterEmail")]
    pub reporter_email: Option<String>,

    #[serde(rename = "showFlagLink")]
    pub show_flag_link: Option<bool>,

    #[serde(rename = "uniqueId")]
    pub unique_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRank {
    #[serde(rename = "isTeamLead")]
    pub is_team_lead: Option<bool>,

    #[serde(rename = "isTeamMember")]
    pub is_team_member: Option<bool>,

    #[serde(rename = "prefix")]
    pub prefix: Option<String>,

    #[serde(rename = "suffix")]
    pub suffix: Option<String>,

    #[serde(rename = "teamName")]
    pub team_name: Option<String>,

    #[serde(rename = "teamProfilePath")]
    pub team_profile_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPronouns {
    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "pronouns")]
    pub pronouns: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInfo {
    #[serde(rename = "encodedZuid")]
    pub encoded_zuid: Option<String>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "isEmployee")]
    pub is_employee: Option<bool>,

    #[serde(rename = "isLender")]
    pub is_lender: Option<bool>,

    #[serde(rename = "isPremierAgent")]
    pub is_premier_agent: Option<bool>,

    #[serde(rename = "isProfessional")]
    pub is_professional: Option<bool>,

    #[serde(rename = "isTeamAccount")]
    pub is_team_account: Option<bool>,

    #[serde(rename = "isTeamLead")]
    pub is_team_lead: Option<bool>,

    #[serde(rename = "isTeamMember")]
    pub is_team_member: Option<bool>,

    #[serde(rename = "isZillowGroupEmployee")]
    pub is_zillow_group_employee: Option<bool>,

    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,

    #[serde(rename = "teamName")]
    pub team_name: Option<String>,

    #[serde(rename = "userRating")]
    pub user_rating: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewsData {
    #[serde(rename = "reviews")]
    pub reviews: Option<Vec<Review>>,

    #[serde(rename = "filters")]
    pub filters: Option<Vec<Filter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "serviceTypeIds")]
    pub service_type_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    #[serde(rename = "reviewComment")]
    pub review_comment: Option<String>,

    #[serde(rename = "reviewId")]
    pub review_id: Option<i64>,

    #[serde(rename = "subRatings")]
    pub sub_ratings: Option<Vec<SubRating>>,

    #[serde(rename = "reviewee")]
    pub reviewee: Option<Reviewe>,

    #[serde(rename = "reviewer")]
    pub reviewer: Option<Reviewe>,

    #[serde(rename = "rating")]
    pub rating: Option<i64>,

    #[serde(rename = "createDate")]
    pub create_date: Option<String>,

    #[serde(rename = "rebuttal")]
    pub rebuttal: Option<serde_json::Value>,

    #[serde(rename = "workDescription")]
    pub work_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reviewe {
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,

    #[serde(rename = "firstName")]
    pub first_name: Option<String>,

    #[serde(rename = "lastName")]
    pub last_name: Option<String>,

    #[serde(rename = "suffix")]
    pub suffix: Option<serde_json::Value>,

    #[serde(rename = "showName")]
    pub show_name: Option<bool>,

    #[serde(rename = "profileImageUrlTemplate")]
    pub profile_image_url_template: Option<String>,

    #[serde(rename = "encodedZuid")]
    pub encoded_zuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubRating {
    #[serde(rename = "description")]
    pub description: Option<Description>,

    #[serde(rename = "score")]
    pub score: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoFooter {
    #[serde(rename = "heading")]
    pub heading: Option<String>,

    #[serde(rename = "items")]
    pub items: Option<Vec<Breadcrumb>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceArea {
    #[serde(rename = "regionId")]
    pub region_id: Option<i64>,

    #[serde(rename = "text")]
    pub text: Option<String>,

    #[serde(rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    #[serde(rename = "assetPrefix")]
    pub asset_prefix: Option<String>,

    #[serde(rename = "staticAssetPrefix")]
    pub static_asset_prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Description {
    #[serde(rename = "Local knowledge")]
    LocalKnowledge,

    #[serde(rename = "Negotiation skills")]
    NegotiationSkills,

    #[serde(rename = "Process expertise")]
    ProcessExpertise,

    #[serde(rename = "Responsiveness")]
    Responsiveness,
}
