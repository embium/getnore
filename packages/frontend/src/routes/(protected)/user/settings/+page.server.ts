import type { PageServerLoad } from "./$types";
import { UsersAPI } from "$lib/api/user";
import { redirect } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals, cookies }) => {
  const cookieHeader = cookies
    .getAll()
    .map((cookie) => `${cookie.name}=${cookie.value}`)
    .join("; ");

  if (!locals.user) {
    return redirect(302, "/login");
  }

  const usersAPI = new UsersAPI(cookieHeader, "server");
  const user = await usersAPI.getUserSettings();

  return {
    user,
  };
};
