export default class TimeUtils {

    public static lastSeen(at: number): string {

        const now = new Date().getTime();
        const diff = now - at;

        if (diff < 60000) {
            return "Just now";
        } else if (diff < 3600000) {
            return Math.floor(diff / 60000) + " minutes ago";
        } else if (diff < 86400000) {
            return Math.floor(diff / 3600000) + " hours ago";
        } else if (diff < 604800000) {
            return Math.floor(diff / 86400000) + " days ago";
        } else if (diff < 2629746000) {
            return Math.floor(diff / 604800000) + " weeks ago";
        } else if (diff < 31556952000) {
            return Math.floor(diff / 2629746000) + " months ago";
        } else {
            return Math.floor(diff / 31556952000) + " years ago";
        }

    }

}