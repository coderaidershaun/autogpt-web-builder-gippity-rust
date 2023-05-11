import React, { useState } from "react";
import Logo from "./Logo";

type Props = {
  currentPage: string;
  setCurrentPage: any; // Leave this as any as an unknown setter function will be passed here
};

function Navigation({ setCurrentPage, currentPage }: Props) {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const handleClick = (page: string) => {
    setCurrentPage(page);
    setIsMobileMenuOpen(false);
  };

  return (
    <div className="bg-white">
      <nav className="container mx-auto py-4 px-6">
        <div className="md:flex justify-between">
          <div className="flex justify-between">
            <div className="flex">
              <Logo />
            </div>
            <div className="md:hidden">
              <button
                className="text-gray-700 focus:outline-none"
                onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                  width="28"
                  height="28"
                >
                  <path
                    fillRule="evenodd"
                    d="M4 8h12a1 1 0 000-2H4a1 1 0 100 2zm0 4h12a1 1 0 000-2H4a1 1 0 100 2zm0 4h12a1 1 0 000-2H4a1 1 0 100 2z"
                    clipRule="evenodd"
                  />
                </svg>
              </button>
            </div>
          </div>
          <div
            className={`${
              isMobileMenuOpen ? "block" : "hidden"
            } md:flex md:flex-row md:items-center md:space-x-8`}
          >
            <a
              className={`${
                currentPage === "home_page" ? "text-blue-500" : "text-gray-500"
              } cursor-pointer py-2 px-4 text-lg md:hover:text-blue-500 md:focus:text-blue-500 md:focus:outline-none`}
              onClick={() => handleClick("home_page")}
            >
              Home
            </a>
            <a
              className={`${
                currentPage === "about_page" ? "text-blue-500" : "text-gray-500"
              } cursor-pointer py-2 px-4 text-lg md:hover:text-blue-500 md:focus:text-blue-500 md:focus:outline-none`}
              onClick={() => handleClick("about_page")}
            >
              About
            </a>
          </div>
        </div>
      </nav>
    </div>
  );
}

export default Navigation;