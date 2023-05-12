import { useState } from "react";

type Props = {
   currentPage: string;
   setCurrentPage: any;
 }

function Footer({setCurrentPage, currentPage}: Props) {
   const gotoHomePage = () => {
      setCurrentPage("home_page");
   }

   const gotoAboutPage = () => {
      setCurrentPage("about_page");
   }

   const linkClass = "p-4 hover:underline cursor-pointer";
   const homeLinkClass = `${
      currentPage === "home_page" ? "underline font-bold" : ""
   } ${linkClass}`;
   const aboutLinkClass = `${
      currentPage === "about_page" ? "underline font-bold" : ""
   } ${linkClass}`;

   return (
     <div className="fixed bottom-0 w-full bg-white border-t border-gray-200 py-3 md:py-6 flex flex-wrap items-center justify-center text-xs md:text-sm">
       <span
         className={homeLinkClass}
         onClick={gotoHomePage}
       >
         Home
       </span>
       <span
         className={aboutLinkClass}
         onClick={gotoAboutPage}
       >
         About
       </span>
     </div>
   )
 }

 export default Footer;