import Logo from "./Logo";

type Props = {
  currentPage: string;
  setCurrentPage: any;
}

function Navigation({ setCurrentPage, currentPage }: Props) {
  return (
    <div className="flex items-center justify-between px-4 py-2 bg-gray-800">
      <div className="flex items-center">
        <Logo />
        <div className="hidden md:flex items-center space-x-4">
          <button
            className={`text-white px-2 ${
              currentPage === "home_page" ? "border-b-2 border-red-500" : ""
            }`}
            onClick={() => setCurrentPage("home_page")}
          >
            Home
          </button>
          <button
            className={`text-white px-2 ${
              currentPage === "about_page" ? "border-b-2 border-red-500" : ""
            }`}
            onClick={() => setCurrentPage("about_page")}
          >
            About
          </button>
        </div>
      </div>
      <div className="md:hidden">
        <button>
          <svg
            className="w-6 h-6 text-white"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fillRule="evenodd"
              d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 6a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm11 6a1 1 0 100-2H4a1 1 0 100 2h10zm3-1a1 1 0 100-2H3a1 1 0 100 2h14z"
              clipRule="evenodd"
            ></path>
          </svg>
        </button>
      </div>
      <div className="fixed top-0 right-0 w-64 h-screen bg-gray-800 transform -translate-x-full md:hidden">
        <button>
          <svg
            className="w-6 h-6 text-white absolute top-4 right-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fillRule="evenodd"
              d="M6.707 5.293a1 1 0 00-1.414 1.414L7.586 9l-2.293 2.293a1 1 0 101.414 1.414L9 10.414l2.293 2.293a1 1 0 001.414-1.414L10.414 9l2.293-2.293a1 1 0 00-1.414-1.414L9 7.586 6.707 5.293z"
              clipRule="evenodd"
            ></path>
          </svg>
        </button>
        <button
          className={`text-white block my-2 ${
            currentPage === "home_page" ? "border-l-2 border-red-500 pl-2" : ""
          }`}
          onClick={() => setCurrentPage("home_page")}
        >
          Home
        </button>
        <button
          className={`text-white block my-2 ${
            currentPage === "about_page" ? "border-l-2 border-red-500 pl-2" : ""
          }`}
          onClick={() => setCurrentPage("about_page")}
        >
          About
        </button>
      </div>
    </div>
  );
}

export default Navigation;